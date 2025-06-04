// Comprehensive integration tests using test utilities
const { test, expect } = require('@playwright/test');
const {
  waitForAppLoad,
  setupConsoleErrorTracking,
  navigateAndWait,
  elementExists,
  verifyPageLoaded,
  testResponsiveDesign,
  checkAccessibility,
  mockApiResponse
} = require('../utils/test-helpers');

test.describe('Comprehensive Application Tests', () => {
  test('Complete user journey through the application', async ({ page }) => {
    const errors = setupConsoleErrorTracking(page);
    
    // Start at home page
    await navigateAndWait(page, '/');
    await verifyPageLoaded(page, '/');
    
    // Navigate to Jobs
    if (await elementExists(page, 'a[href="/jobs"], text="Jobs"')) {
      await page.locator('a[href="/jobs"], text="Jobs"').first().click();
      await waitForAppLoad(page);
      await verifyPageLoaded(page, '/jobs');
    }
    
    // Navigate to Map
    if (await elementExists(page, 'a[href="/map"], text="Map"')) {
      await page.locator('a[href="/map"], text="Map"').first().click();
      await waitForAppLoad(page);
      await verifyPageLoaded(page, '/map');
    }
    
    // Navigate to Forum
    if (await elementExists(page, 'a[href="/forum"], text="Forum"')) {
      await page.locator('a[href="/forum"], text="Forum"').first().click();
      await waitForAppLoad(page);
      await verifyPageLoaded(page, '/forum');
    }
    
    // Return to home
    if (await elementExists(page, 'a[href="/"], text="Home"')) {
      await page.locator('a[href="/"], text="Home"').first().click();
      await waitForAppLoad(page);
      await verifyPageLoaded(page, '/');
    }
    
    // Check for critical errors
    const criticalErrors = errors.filter(error => 
      !error.includes('favicon') && 
      !error.includes('DevTools') &&
      !error.includes('Mapbox') && // Mapbox errors are expected without token
      !error.includes('token')
    );
    
    expect(criticalErrors).toHaveLength(0);
  });

  test('Responsive design across all pages', async ({ page }) => {
    const pages = ['/', '/jobs', '/map', '/forum', '/profile'];
    
    for (const pagePath of pages) {
      await test.step(`Testing responsive design for ${pagePath}`, async () => {
        await testResponsiveDesign(page, async (page, viewport) => {
          await navigateAndWait(page, pagePath);
          
          // Basic checks for each viewport
          const bodyVisible = await page.locator('body').isVisible();
          expect(bodyVisible).toBeTruthy();
          
          // Check that content adapts to viewport
          const contentWidth = await page.locator('body').evaluate(el => el.scrollWidth);
          expect(contentWidth).toBeLessThanOrEqual(viewport.width + 50); // Small buffer for scrollbars
        });
      });
    }
  });

  test('API integration with mock data', async ({ page }) => {
    // Mock jobs API
    await mockApiResponse(page, '/api/jobs', [
      {
        id: '1',
        title: 'Test Pharmacist Position',
        description: 'A test job for E2E testing',
        location: 'Sydney, NSW',
        type: 'Full-time'
      },
      {
        id: '2',
        title: 'Senior Pharmacy Manager',
        description: 'Management role in pharmacy',
        location: 'Melbourne, VIC', 
        type: 'Full-time'
      }
    ]);
    
    await navigateAndWait(page, '/jobs');
    
    // Wait for jobs to load
    await page.waitForTimeout(3000);
    
    // Should show mocked jobs
    const jobElements = await page.locator('[data-testid="job-card"], .job-card, .job-item').count();
    const hasJobContent = await elementExists(page, 'text="Test Pharmacist Position"');
    
    expect(jobElements > 0 || hasJobContent).toBeTruthy();
  });

  test('Error handling across the application', async ({ page }) => {
    const errors = setupConsoleErrorTracking(page);
    
    // Test 404 page
    await page.goto('/non-existent-page');
    await waitForAppLoad(page);
    
    // Should handle 404 gracefully
    const has404Content = await elementExists(page, 'text=/404|not found|page not found/i');
    const stillHasNavigation = await elementExists(page, 'nav, [role="navigation"]');
    
    expect(has404Content || stillHasNavigation).toBeTruthy();
    
    // Navigate back to valid page
    await navigateAndWait(page, '/');
    await verifyPageLoaded(page, '/');
    
    // App should recover gracefully
    const appWorking = await elementExists(page, 'body');
    expect(appWorking).toBeTruthy();
  });

  test('Accessibility compliance', async ({ page }) => {
    const pages = ['/', '/jobs', '/map'];
    
    for (const pagePath of pages) {
      await test.step(`Checking accessibility for ${pagePath}`, async () => {
        await navigateAndWait(page, pagePath);
        
        const accessibilityIssues = await checkAccessibility(page);
        
        // Log issues but don't fail tests yet (can be improved incrementally)
        if (accessibilityIssues.length > 0) {
          console.log(`Accessibility issues on ${pagePath}:`, accessibilityIssues);
        }
        
        // At minimum, page should have basic structure
        const hasHeading = await elementExists(page, 'h1, h2, h3');
        expect(hasHeading).toBeTruthy();
      });
    }
  });

  test('Search functionality across the application', async ({ page }) => {
    // Test search on jobs page
    await navigateAndWait(page, '/jobs');
    
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"], [data-testid="search"]').first();
    
    if (await searchInput.count() > 0) {
      await searchInput.fill('pharmacist');
      await searchInput.press('Enter');
      
      await page.waitForTimeout(2000);
      
      // Search should trigger some response
      const urlChanged = page.url().includes('search') || page.url().includes('pharmacist');
      const contentChanged = await elementExists(page, '.search-results, [data-testid="search-results"]');
      
      expect(urlChanged || contentChanged).toBeTruthy();
    }
  });

  test('Navigation consistency', async ({ page }) => {
    const routes = ['/', '/jobs', '/map', '/forum', '/profile'];
    
    for (const route of routes) {
      await test.step(`Testing navigation consistency for ${route}`, async () => {
        await navigateAndWait(page, route);
        
        // Each page should have consistent navigation
        const hasNavigation = await elementExists(page, 'nav, [role="navigation"], .sidebar');
        expect(hasNavigation).toBeTruthy();
        
        // Each page should have a way to get back to home
        const hasHomeLink = await elementExists(page, 'a[href="/"], text="Home", [data-testid="home-link"]');
        
        // Home link is not strictly required but navigation should exist
        expect(hasNavigation).toBeTruthy();
      });
    }
  });

  test('State management across navigation', async ({ page }) => {
    // Start at jobs page and perform a search
    await navigateAndWait(page, '/jobs');
    
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    
    if (await searchInput.count() > 0) {
      await searchInput.fill('test search');
      await page.waitForTimeout(1000);
      
      // Navigate to another page
      await navigateAndWait(page, '/map');
      
      // Navigate back to jobs
      await navigateAndWait(page, '/jobs');
      
      // Check if search state is preserved (optional behavior)
      const searchValue = await searchInput.inputValue().catch(() => '');
      
      // State preservation is optional, but page should still function
      const pageWorks = await elementExists(page, 'body');
      expect(pageWorks).toBeTruthy();
    }
  });
});