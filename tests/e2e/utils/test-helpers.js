// Test utilities and helper functions
const { expect } = require('@playwright/test');

/**
 * Wait for the application to fully load
 */
async function waitForAppLoad(page, timeout = 10000) {
  await page.waitForLoadState('networkidle');
  
  // Wait for any WASM to load
  await page.waitForTimeout(2000);
  
  // Check if app has loaded by looking for body content
  await expect(page.locator('body')).toBeVisible({ timeout });
}

/**
 * Check for console errors and filter out known harmless ones
 */
function setupConsoleErrorTracking(page) {
  const errors = [];
  
  page.on('console', (msg) => {
    if (msg.type() === 'error') {
      const text = msg.text();
      
      // Filter out known harmless errors
      if (!text.includes('favicon') && 
          !text.includes('DevTools') && 
          !text.includes('chrome-extension') &&
          !text.includes('extension')) {
        errors.push(text);
        console.error(`Console error: ${text}`);
      }
    }
  });
  
  page.on('pageerror', (exception) => {
    errors.push(exception.toString());
    console.error(`Uncaught exception: ${exception}`);
  });
  
  return errors;
}

/**
 * Navigate to a page and wait for it to load
 */
async function navigateAndWait(page, url) {
  await page.goto(url);
  await waitForAppLoad(page);
}

/**
 * Check if an element exists without throwing
 */
async function elementExists(page, selector) {
  try {
    return await page.locator(selector).count() > 0;
  } catch (error) {
    // If selector is invalid, try treating it as text content
    if (error.message.includes('while parsing css selector')) {
      // Extract text from quotes if it's a text selector
      const textMatch = selector.match(/text="([^"]+)"/);
      if (textMatch) {
        return await page.getByText(textMatch[1]).count() > 0;
      }
      // If it contains multiple selectors separated by commas
      if (selector.includes(',')) {
        const selectors = selector.split(',').map(s => s.trim());
        for (const sel of selectors) {
          try {
            const count = await page.locator(sel).count();
            if (count > 0) return true;
          } catch {
            // Try as text if selector fails
            const textMatch = sel.match(/text="([^"]+)"/);
            if (textMatch) {
              const textCount = await page.getByText(textMatch[1]).count();
              if (textCount > 0) return true;
            }
          }
        }
        return false;
      }
    }
    return false;
  }
}

/**
 * Wait for API calls to complete
 */
async function waitForApiCalls(page, timeout = 5000) {
  let apiCallsInProgress = 0;
  
  page.on('request', request => {
    if (request.url().includes('/api/')) {
      apiCallsInProgress++;
    }
  });
  
  page.on('response', response => {
    if (response.url().includes('/api/')) {
      apiCallsInProgress--;
    }
  });
  
  const startTime = Date.now();
  while (apiCallsInProgress > 0 && (Date.now() - startTime) < timeout) {
    await page.waitForTimeout(100);
  }
}

/**
 * Simulate slow network for performance testing
 */
async function enableSlowNetwork(page) {
  const client = await page.context().newCDPSession(page);
  await client.send('Network.enable');
  await client.send('Network.emulateNetworkConditions', {
    offline: false,
    downloadThroughput: 500 * 1024, // 500 KB/s
    uploadThroughput: 500 * 1024,   // 500 KB/s
    latency: 100 // 100ms latency
  });
}

/**
 * Take a screenshot with timestamp
 */
async function takeScreenshot(page, name) {
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
  const filename = `${name}-${timestamp}.png`;
  await page.screenshot({ path: `screenshots/${filename}`, fullPage: true });
  return filename;
}

/**
 * Check if the page has loaded successfully
 */
async function verifyPageLoaded(page, expectedPath) {
  // Check URL
  const currentUrl = page.url();
  expect(currentUrl).toContain(expectedPath);
  
  // Check that page is responsive
  const bodyExists = await elementExists(page, 'body');
  expect(bodyExists).toBeTruthy();
  
  // Check that no major errors occurred
  const hasErrorPage = await elementExists(page, '.error-page, [data-testid="error-page"]');
  expect(hasErrorPage).toBeFalsy();
}

/**
 * Test responsive design at different viewport sizes
 */
async function testResponsiveDesign(page, test) {
  const viewports = [
    { width: 1920, height: 1080, name: 'Desktop' },
    { width: 1024, height: 768, name: 'Tablet' },
    { width: 375, height: 667, name: 'Mobile' }
  ];
  
  for (const viewport of viewports) {
    await page.setViewportSize({ width: viewport.width, height: viewport.height });
    await page.waitForTimeout(1000); // Allow layout to settle
    
    console.log(`Testing ${viewport.name} (${viewport.width}x${viewport.height})`);
    await test(page, viewport);
  }
}

/**
 * Mock API responses for testing
 */
async function mockApiResponse(page, endpoint, response) {
  await page.route(`**${endpoint}**`, route => {
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify(response)
    });
  });
}

/**
 * Mock API error for testing error states
 */
async function mockApiError(page, endpoint, statusCode = 500) {
  await page.route(`**${endpoint}**`, route => {
    route.fulfill({
      status: statusCode,
      contentType: 'application/json',
      body: JSON.stringify({ error: 'Mock API error' })
    });
  });
}

/**
 * Test keyboard navigation
 */
async function testKeyboardNavigation(page, startSelector) {
  if (startSelector && await elementExists(page, startSelector)) {
    await page.locator(startSelector).focus();
  }
  
  const focusableElements = [];
  
  // Tab through elements
  for (let i = 0; i < 10; i++) {
    await page.keyboard.press('Tab');
    await page.waitForTimeout(100);
    
    const focusedElement = page.locator(':focus');
    const tagName = await focusedElement.evaluate(el => el.tagName).catch(() => 'UNKNOWN');
    
    if (tagName !== 'UNKNOWN') {
      focusableElements.push(tagName);
    }
  }
  
  return focusableElements;
}

/**
 * Check for accessibility violations
 */
async function checkAccessibility(page) {
  // Basic accessibility checks
  const issues = [];
  
  // Check for images without alt text
  const imagesWithoutAlt = await page.locator('img:not([alt])').count();
  if (imagesWithoutAlt > 0) {
    issues.push(`${imagesWithoutAlt} images without alt text`);
  }
  
  // Check for inputs without labels
  const inputsWithoutLabels = await page.locator('input:not([aria-label]):not([aria-labelledby])').count();
  if (inputsWithoutLabels > 0) {
    issues.push(`${inputsWithoutLabels} inputs without proper labels`);
  }
  
  // Check for proper heading structure
  const h1Count = await page.locator('h1').count();
  if (h1Count === 0) {
    issues.push('No h1 heading found');
  } else if (h1Count > 1) {
    issues.push('Multiple h1 headings found');
  }
  
  return issues;
}

module.exports = {
  waitForAppLoad,
  setupConsoleErrorTracking,
  navigateAndWait,
  elementExists,
  waitForApiCalls,
  enableSlowNetwork,
  takeScreenshot,
  verifyPageLoaded,
  testResponsiveDesign,
  mockApiResponse,
  mockApiError,
  testKeyboardNavigation,
  checkAccessibility
};