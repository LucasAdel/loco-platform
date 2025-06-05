const { test, expect } = require('@playwright/test');

test.describe('Debug Test', () => {
  test('Check forum page content', async ({ page }) => {
    // Start server manually for testing
    await page.goto('http://localhost:8000/forum');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // Give JS time to run
    
    // Log everything we can see
    const content = await page.content();
    console.log('PAGE CONTENT:', content.substring(0, 2000));
    
    // Check specific selectors
    const forumHeading = await page.locator('h2:has-text("Forum")').count();
    console.log('Forum heading count:', forumHeading);
    
    const comingSoon = await page.locator('text=/coming soon/i').count();
    console.log('Coming soon count:', comingSoon);
    
    const allText = await page.locator('text=/forum/i').count();
    console.log('Forum text count:', allText);
    
    // Take screenshot
    await page.screenshot({ path: 'debug_forum.png' });
  });
});