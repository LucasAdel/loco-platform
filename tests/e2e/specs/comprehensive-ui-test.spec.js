// @ts-check
const { test, expect } = require('@playwright/test');

// Define all pages to test
const PAGES_TO_TEST = [
  { path: '/index.html', name: 'Home Page' },
  { path: '/dashboard.html', name: 'Dashboard' },
  { path: '/dashboard-2025.html', name: 'Dashboard 2025' },
  { path: '/jobs.html', name: 'Jobs List' },
  { path: '/create-job.html', name: 'Create Job' },
  { path: '/create-job-enhanced.html', name: 'Create Job Enhanced' },
  { path: '/job-detail.html', name: 'Job Detail' },
  { path: '/map.html', name: 'Map' },
  { path: '/profile.html', name: 'Profile' },
  { path: '/login.html', name: 'Login' },
  { path: '/register.html', name: 'Register' },
  { path: '/admin.html', name: 'Admin' },
];

// Helper function to check console errors
async function checkConsoleErrors(page, pageName) {
  const consoleErrors = [];
  const consoleWarnings = [];
  
  page.on('console', msg => {
    if (msg.type() === 'error') {
      consoleErrors.push(msg.text());
    } else if (msg.type() === 'warning') {
      consoleWarnings.push(msg.text());
    }
  });
  
  page.on('pageerror', err => {
    consoleErrors.push(err.message);
  });
  
  return { consoleErrors, consoleWarnings };
}

// Helper function to check viewport responsiveness
async function checkResponsiveness(page, pageName) {
  const viewports = [
    { name: 'Mobile', width: 375, height: 667 },
    { name: 'Tablet', width: 768, height: 1024 },
    { name: 'Desktop', width: 1920, height: 1080 },
  ];
  
  const issues = [];
  
  for (const viewport of viewports) {
    await page.setViewportSize({ width: viewport.width, height: viewport.height });
    
    // Check for horizontal overflow
    const horizontalOverflow = await page.evaluate(() => {
      return document.documentElement.scrollWidth > window.innerWidth;
    });
    
    if (horizontalOverflow) {
      issues.push(`${viewport.name}: Horizontal overflow detected`);
    }
    
    // Check for overlapping elements
    const overlapping = await page.evaluate(() => {
      const elements = document.querySelectorAll('*');
      const overlaps = [];
      
      for (let i = 0; i < elements.length; i++) {
        const rect1 = elements[i].getBoundingClientRect();
        if (rect1.width === 0 || rect1.height === 0) continue;
        
        for (let j = i + 1; j < elements.length; j++) {
          const rect2 = elements[j].getBoundingClientRect();
          if (rect2.width === 0 || rect2.height === 0) continue;
          
          // Skip parent-child relationships
          if (elements[i].contains(elements[j]) || elements[j].contains(elements[i])) continue;
          
          // Check for overlap
          if (!(rect1.right < rect2.left || 
                rect2.right < rect1.left || 
                rect1.bottom < rect2.top || 
                rect2.bottom < rect1.top)) {
            overlaps.push({
              elem1: elements[i].tagName + (elements[i].className ? '.' + elements[i].className : ''),
              elem2: elements[j].tagName + (elements[j].className ? '.' + elements[j].className : '')
            });
          }
        }
      }
      
      return overlaps.slice(0, 5); // Return only first 5 overlaps
    });
    
    if (overlapping.length > 0) {
      issues.push(`${viewport.name}: Overlapping elements detected`);
    }
  }
  
  return issues;
}

// Helper function to check form elements
async function checkFormElements(page) {
  const issues = [];
  
  // Check all inputs have proper border radius
  const inputsWithoutSoftCorners = await page.evaluate(() => {
    const inputs = document.querySelectorAll('input, textarea, select, button');
    const problematic = [];
    
    inputs.forEach(input => {
      const styles = window.getComputedStyle(input);
      const borderRadius = styles.borderRadius;
      
      // Check if border radius is less than 8px (soft corners should be 12px ideally)
      if (!borderRadius || parseInt(borderRadius) < 8) {
        problematic.push({
          tag: input.tagName,
          id: input.id || 'no-id',
          class: input.className || 'no-class',
          borderRadius: borderRadius
        });
      }
    });
    
    return problematic;
  });
  
  if (inputsWithoutSoftCorners.length > 0) {
    issues.push(`Found ${inputsWithoutSoftCorners.length} form elements without soft corners (12px border-radius)`);
  }
  
  // Check all form inputs have labels
  const inputsWithoutLabels = await page.evaluate(() => {
    const inputs = document.querySelectorAll('input:not([type="submit"]):not([type="button"]), textarea, select');
    const problematic = [];
    
    inputs.forEach(input => {
      const id = input.id;
      const ariaLabel = input.getAttribute('aria-label');
      const ariaLabelledBy = input.getAttribute('aria-labelledby');
      const label = id ? document.querySelector(`label[for="${id}"]`) : null;
      
      if (!label && !ariaLabel && !ariaLabelledBy) {
        problematic.push({
          tag: input.tagName,
          id: id || 'no-id',
          type: input.type || 'no-type'
        });
      }
    });
    
    return problematic;
  });
  
  if (inputsWithoutLabels.length > 0) {
    issues.push(`Found ${inputsWithoutLabels.length} form inputs without proper labels`);
  }
  
  return issues;
}

// Helper function to check accessibility
async function checkAccessibility(page) {
  const issues = [];
  
  // Check images have alt text
  const imagesWithoutAlt = await page.evaluate(() => {
    const images = document.querySelectorAll('img');
    let count = 0;
    images.forEach(img => {
      if (!img.alt && !img.getAttribute('aria-label')) {
        count++;
      }
    });
    return count;
  });
  
  if (imagesWithoutAlt > 0) {
    issues.push(`Found ${imagesWithoutAlt} images without alt text`);
  }
  
  // Check headings hierarchy
  const headingIssues = await page.evaluate(() => {
    const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
    const issues = [];
    let lastLevel = 0;
    
    headings.forEach(h => {
      const level = parseInt(h.tagName.charAt(1));
      if (lastLevel > 0 && level > lastLevel + 1) {
        issues.push(`Heading hierarchy issue: ${h.tagName} follows H${lastLevel}`);
      }
      lastLevel = level;
    });
    
    return issues;
  });
  
  if (headingIssues.length > 0) {
    issues.push(...headingIssues);
  }
  
  // Check buttons have accessible text
  const buttonsWithoutText = await page.evaluate(() => {
    const buttons = document.querySelectorAll('button, a[role="button"], input[type="submit"], input[type="button"]');
    let count = 0;
    
    buttons.forEach(btn => {
      const text = btn.textContent?.trim();
      const ariaLabel = btn.getAttribute('aria-label');
      const title = btn.getAttribute('title');
      
      if (!text && !ariaLabel && !title) {
        count++;
      }
    });
    
    return count;
  });
  
  if (buttonsWithoutText > 0) {
    issues.push(`Found ${buttonsWithoutText} buttons without accessible text`);
  }
  
  return issues;
}

// Main test suite
test.describe('Comprehensive UI Testing', () => {
  // Test each page
  for (const pageInfo of PAGES_TO_TEST) {
    test.describe(pageInfo.name, () => {
      test('should load successfully without errors', async ({ page }) => {
        const { consoleErrors, consoleWarnings } = await checkConsoleErrors(page, pageInfo.name);
        
        // Navigate to page
        const response = await page.goto(pageInfo.path);
        
        // Check page loads successfully
        expect(response.status()).toBe(200);
        
        // Wait for page to fully load
        await page.waitForLoadState('networkidle');
        
        // Check for console errors
        expect(consoleErrors).toHaveLength(0);
        
        // Take screenshot for visual reference
        await page.screenshot({ 
          path: `test-results/${pageInfo.name.replace(/\s+/g, '-')}-screenshot.png`,
          fullPage: true 
        });
      });
      
      test('should be responsive across viewports', async ({ page }) => {
        await page.goto(pageInfo.path);
        await page.waitForLoadState('networkidle');
        
        const responsiveIssues = await checkResponsiveness(page, pageInfo.name);
        expect(responsiveIssues).toHaveLength(0);
      });
      
      test('should have proper form elements', async ({ page }) => {
        await page.goto(pageInfo.path);
        await page.waitForLoadState('networkidle');
        
        const formIssues = await checkFormElements(page);
        expect(formIssues).toHaveLength(0);
      });
      
      test('should meet accessibility standards', async ({ page }) => {
        await page.goto(pageInfo.path);
        await page.waitForLoadState('networkidle');
        
        const accessibilityIssues = await checkAccessibility(page);
        expect(accessibilityIssues).toHaveLength(0);
      });
      
      test('should have working navigation links', async ({ page }) => {
        await page.goto(pageInfo.path);
        await page.waitForLoadState('networkidle');
        
        // Check all internal links
        const brokenLinks = await page.evaluate(async () => {
          const links = document.querySelectorAll('a[href^="/"], a[href^="./"], a[href^="../"]');
          const broken = [];
          
          for (const link of links) {
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#')) {
              try {
                const response = await fetch(href, { method: 'HEAD' });
                if (!response.ok) {
                  broken.push({ href, status: response.status });
                }
              } catch (err) {
                broken.push({ href, error: err.message });
              }
            }
          }
          
          return broken;
        });
        
        expect(brokenLinks).toHaveLength(0);
      });
    });
  }
  
  // Cross-page navigation test
  test('should navigate between pages correctly', async ({ page }) => {
    // Start at home page
    await page.goto('/index.html');
    
    // Test navigation to different pages
    const navigationTests = [
      { linkText: 'Dashboard', expectedUrl: /dashboard\.html/ },
      { linkText: 'Jobs', expectedUrl: /jobs\.html/ },
      { linkText: 'Map', expectedUrl: /map\.html/ },
      { linkText: 'Profile', expectedUrl: /profile\.html/ },
    ];
    
    for (const navTest of navigationTests) {
      // Go back to home
      await page.goto('/index.html');
      
      // Try to find and click the link
      const link = page.locator(`a:has-text("${navTest.linkText}")`).first();
      
      if (await link.count() > 0) {
        await link.click();
        await expect(page).toHaveURL(navTest.expectedUrl);
      }
    }
  });
});

// Test for specific UI components
test.describe('UI Component Testing', () => {
  test('all buttons should have hover states', async ({ page }) => {
    await page.goto('/index.html');
    
    const buttonsWithoutHover = await page.evaluate(() => {
      const buttons = document.querySelectorAll('button, .btn, a.button');
      const problematic = [];
      
      buttons.forEach(btn => {
        const normalBg = window.getComputedStyle(btn).backgroundColor;
        
        // Simulate hover
        btn.classList.add('hover');
        const hoverBg = window.getComputedStyle(btn).backgroundColor;
        btn.classList.remove('hover');
        
        if (normalBg === hoverBg) {
          problematic.push({
            element: btn.tagName,
            class: btn.className,
            text: btn.textContent?.trim()
          });
        }
      });
      
      return problematic;
    });
    
    expect(buttonsWithoutHover).toHaveLength(0);
  });
  
  test('forms should have proper validation', async ({ page }) => {
    // Test login form validation
    await page.goto('/login.html');
    
    // Try to submit empty form
    const submitButton = page.locator('button[type="submit"], input[type="submit"]').first();
    if (await submitButton.count() > 0) {
      await submitButton.click();
      
      // Check for validation messages
      const validationMessages = await page.evaluate(() => {
        const inputs = document.querySelectorAll('input[required]');
        const messages = [];
        
        inputs.forEach(input => {
          if (input.validationMessage) {
            messages.push({
              field: input.name || input.id,
              message: input.validationMessage
            });
          }
        });
        
        return messages;
      });
      
      // Required fields should show validation messages
      expect(validationMessages.length).toBeGreaterThan(0);
    }
  });
});

// Performance tests
test.describe('Performance Testing', () => {
  test('pages should load within acceptable time', async ({ page }) => {
    const performanceResults = [];
    
    for (const pageInfo of PAGES_TO_TEST) {
      const startTime = Date.now();
      await page.goto(pageInfo.path);
      await page.waitForLoadState('networkidle');
      const loadTime = Date.now() - startTime;
      
      performanceResults.push({
        page: pageInfo.name,
        loadTime: loadTime
      });
      
      // Page should load within 3 seconds
      expect(loadTime).toBeLessThan(3000);
    }
    
    console.log('Performance Results:', performanceResults);
  });
  
  test('images should be optimized', async ({ page }) => {
    await page.goto('/index.html');
    
    const largeImages = await page.evaluate(() => {
      const images = document.querySelectorAll('img');
      const large = [];
      
      images.forEach(img => {
        if (img.naturalWidth > 2000 || img.naturalHeight > 2000) {
          large.push({
            src: img.src,
            width: img.naturalWidth,
            height: img.naturalHeight
          });
        }
      });
      
      return large;
    });
    
    expect(largeImages).toHaveLength(0);
  });
});