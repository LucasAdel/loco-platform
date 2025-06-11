import { test, expect } from '@playwright/test';

test.describe('Quick App Check', () => {
  test('Check if app is serving', async ({ page }) => {
    // Navigate to home page
    await page.goto('http://localhost:3080');
    
    // Wait for app to load
    await page.waitForTimeout(2000);
    
    // Take a screenshot
    await page.screenshot({ path: 'app-state.png' });
    
    // Check what's on the page
    const bodyText = await page.textContent('body');
    console.log('Page content:', bodyText?.substring(0, 500));
    
    // Check for error overlay
    const hasError = await page.locator('text=error').count();
    console.log('Error elements found:', hasError);
    
    // Check if WASM loaded
    const wasmLoaded = await page.evaluate(() => {
      return typeof window.wasmBindings !== 'undefined';
    });
    console.log('WASM loaded:', wasmLoaded);
    
    // Check for any Leptos components
    const hasComponents = await page.locator('[data-hk]').count();
    console.log('Leptos components found:', hasComponents);
  });
});