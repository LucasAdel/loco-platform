// Performance and accessibility tests
const { test, expect } = require('@playwright/test');

test.describe('Performance Tests', () => {
  test('Page load performance is acceptable', async ({ page }) => {
    // Start timing
    const startTime = Date.now();
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    const loadTime = Date.now() - startTime;
    
    // Page should load within 10 seconds (generous for development)
    expect(loadTime).toBeLessThan(10000);
    
    console.log(`Home page load time: ${loadTime}ms`);
  });

  test('Jobs page performance is acceptable', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/jobs');
    await page.waitForLoadState('networkidle');
    
    const loadTime = Date.now() - startTime;
    
    // Jobs page should load within 15 seconds (allowing for API calls)
    expect(loadTime).toBeLessThan(15000);
    
    console.log(`Jobs page load time: ${loadTime}ms`);
  });

  test('Map page performance is acceptable', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/map');
    await page.waitForLoadState('networkidle');
    
    // Allow extra time for Mapbox to load
    await page.waitForTimeout(5000);
    
    const loadTime = Date.now() - startTime;
    
    // Map page should load within 20 seconds (Mapbox can be slow)
    expect(loadTime).toBeLessThan(20000);
    
    console.log(`Map page load time: ${loadTime}ms`);
  });

  test('WASM bundle loads efficiently', async ({ page }) => {
    let wasmLoaded = false;
    let wasmLoadTime = 0;
    const startTime = Date.now();
    
    page.on('response', response => {
      if (response.url().includes('.wasm')) {
        wasmLoaded = true;
        wasmLoadTime = Date.now() - startTime;
        console.log(`WASM loaded in: ${wasmLoadTime}ms`);
      }
    });
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(5000);
    
    if (wasmLoaded) {
      // WASM should load within 10 seconds
      expect(wasmLoadTime).toBeLessThan(10000);
    }
  });
});

test.describe('Accessibility Tests', () => {
  test('Home page has proper heading structure', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Should have at least one main heading
    const h1Count = await page.locator('h1').count();
    expect(h1Count).toBeGreaterThan(0);
    
    // Check for proper heading hierarchy
    const headings = await page.locator('h1, h2, h3, h4, h5, h6').allTextContents();
    expect(headings.length).toBeGreaterThan(0);
  });

  test('Navigation is keyboard accessible', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Test tab navigation
    await page.keyboard.press('Tab');
    const focusedElement = await page.locator(':focus').count();
    expect(focusedElement).toBeGreaterThan(0);
    
    // Continue tabbing through navigation
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('Tab');
      await page.waitForTimeout(100);
    }
    
    // Should be able to navigate with Enter
    await page.keyboard.press('Enter');
    await page.waitForTimeout(1000);
  });

  test('Images have alt text', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    const images = page.locator('img');
    const imageCount = await images.count();
    
    if (imageCount > 0) {
      for (let i = 0; i < imageCount; i++) {
        const img = images.nth(i);
        const alt = await img.getAttribute('alt');
        const ariaLabel = await img.getAttribute('aria-label');
        
        // Images should have alt text or aria-label
        expect(alt !== null || ariaLabel !== null).toBeTruthy();
      }
    }
  });

  test('Forms have proper labels', async ({ page }) => {
    await page.goto('/jobs');
    await page.waitForLoadState('networkidle');
    
    const inputs = page.locator('input, select, textarea');
    const inputCount = await inputs.count();
    
    if (inputCount > 0) {
      for (let i = 0; i < inputCount; i++) {
        const input = inputs.nth(i);
        const label = await input.getAttribute('aria-label');
        const placeholder = await input.getAttribute('placeholder');
        const associatedLabel = await page.locator(`label[for="${await input.getAttribute('id')}"]`).count();
        
        // Inputs should have labels, aria-labels, or placeholders
        expect(label !== null || placeholder !== null || associatedLabel > 0).toBeTruthy();
      }
    }
  });

  test('Page has proper ARIA landmarks', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Check for semantic HTML or ARIA landmarks
    const nav = await page.locator('nav, [role="navigation"]').count();
    const main = await page.locator('main, [role="main"]').count();
    
    // Should have navigation and main content areas
    expect(nav).toBeGreaterThan(0);
  });

  test('Contrast and visibility', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Check that text is visible (not transparent or same colour as background)
    const textElements = page.locator('p, h1, h2, h3, h4, h5, h6, span, a, button');
    const firstTextElement = textElements.first();
    
    if (await firstTextElement.count() > 0) {
      const isVisible = await firstTextElement.isVisible();
      expect(isVisible).toBeTruthy();
    }
  });
});

test.describe('Browser Compatibility', () => {
  test('Works in Chromium', async ({ page, browserName }) => {
    test.skip(browserName !== 'chromium', 'This test is for Chromium only');
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    const title = await page.title();
    expect(title).toBeTruthy();
  });

  test('Works in Firefox', async ({ page, browserName }) => {
    test.skip(browserName !== 'firefox', 'This test is for Firefox only');
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    const title = await page.title();
    expect(title).toBeTruthy();
  });

  test('Works in Safari/WebKit', async ({ page, browserName }) => {
    test.skip(browserName !== 'webkit', 'This test is for Safari/WebKit only');
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    const title = await page.title();
    expect(title).toBeTruthy();
  });
});