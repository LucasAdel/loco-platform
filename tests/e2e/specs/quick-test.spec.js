// @ts-check
const { test, expect } = require('@playwright/test');

const CRITICAL_PAGES = [
  { path: '/login.html', name: 'Login' },
  { path: '/jobs.html', name: 'Jobs' },
  { path: '/profile.html', name: 'Profile' },
  { path: '/dashboard.html', name: 'Dashboard' },
  { path: '/job-detail.html', name: 'Job Detail' }
];

test.describe('Quick Fix Verification', () => {
  for (const page of CRITICAL_PAGES) {
    test(`${page.name} - Core functionality`, async ({ page: browserPage }) => {
      const errors = [];
      
      // Capture console errors
      browserPage.on('console', msg => {
        if (msg.type() === 'error' && !msg.text().includes('cdn.tailwindcss.com')) {
          errors.push(msg.text());
        }
      });
      
      // Navigate to page
      const response = await browserPage.goto(page.path);
      expect(response.status()).toBe(200);
      
      // Wait for content to load
      await browserPage.waitForLoadState('networkidle');
      
      // Check if supabase initialization errors are fixed
      const supabaseErrors = errors.filter(err => 
        err.includes('Cannot read properties of null') && 
        err.includes('supabase')
      );
      expect(supabaseErrors).toHaveLength(0);
      
      // Check for soft corners on inputs
      const inputsWithBadRadius = await browserPage.evaluate(() => {
        const inputs = document.querySelectorAll('input, textarea, select, button');
        const bad = [];
        
        inputs.forEach(input => {
          const styles = window.getComputedStyle(input);
          const radius = styles.borderRadius;
          
          if (radius && parseInt(radius) < 12) {
            bad.push({
              tag: input.tagName,
              radius: radius
            });
          }
        });
        
        return bad;
      });
      
      // All inputs should have 12px border radius
      expect(inputsWithBadRadius).toHaveLength(0);
      
      // Check basic accessibility
      const missingLabels = await browserPage.evaluate(() => {
        const inputs = document.querySelectorAll('input:not([type="submit"]):not([type="button"]), textarea, select');
        let count = 0;
        
        inputs.forEach(input => {
          const id = input.id;
          const ariaLabel = input.getAttribute('aria-label');
          const label = id ? document.querySelector(`label[for="${id}"]`) : null;
          
          if (!label && !ariaLabel) {
            count++;
          }
        });
        
        return count;
      });
      
      // Should have proper labels
      expect(missingLabels).toBe(0);
    });
  }
});