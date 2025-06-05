// Navigation and page loading tests for Loco Platform
const { test, expect } = require('@playwright/test');

test.describe('Page Navigation', () => {
  test.beforeEach(async ({ page }) => {
    // Listen for console errors
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        console.error(`Console error: ${msg.text()}`);
      }
    });

    // Listen for uncaught exceptions
    page.on('pageerror', (exception) => {
      console.error(`Uncaught exception: ${exception}`);
    });
  });

  test('Home page loads successfully', async ({ page }) => {
    await page.goto('/');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check that the page title is present
    await expect(page).toHaveTitle(/Loco Platform/);
    
    // Verify no console errors (check console messages)
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });
    
    // Wait a bit to catch any async errors
    await page.waitForTimeout(2000);
    
    // Should have no critical errors
    expect(errors.filter(error => 
      !error.includes('favicon') && 
      !error.includes('DevTools')
    )).toHaveLength(0);
  });

  test('Jobs page loads successfully', async ({ page }) => {
    await page.goto('/jobs');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on jobs page
    expect(page.url()).toContain('/jobs');
    
    // Check for jobs content or loading state
    const hasJobsContent = await page.locator('[data-testid="jobs-container"], .jobs-container, .job-list').count() > 0;
    const hasLoadingState = await page.locator('[data-testid="loading"], .loading').count() > 0;
    
    // Should have either jobs content or loading state
    expect(hasJobsContent || hasLoadingState).toBeTruthy();
  });

  test('Map page loads successfully', async ({ page }) => {
    await page.goto('/map');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on map page
    expect(page.url()).toContain('/map');
    
    // Give Mapbox time to load
    await page.waitForTimeout(3000);
    
    // Check for map container or loading state
    const hasMapContainer = await page.locator('[data-testid="map-container"], .map-container, #map').count() > 0;
    const hasLoadingState = await page.locator('[data-testid="loading"], .loading').count() > 0;
    
    expect(hasMapContainer || hasLoadingState).toBeTruthy();
  });

  test('Forum page loads successfully', async ({ page }) => {
    await page.goto('/forum');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on forum page
    expect(page.url()).toContain('/forum');
    
    // Check for specific forum content
    const hasForumHeading = await page.locator('h2:has-text("Professional Forum")').count() > 0;
    const hasComingSoon = await page.locator('text=Forum features coming soon').count() > 0;
    
    expect(hasForumHeading || hasComingSoon).toBeTruthy();
  });

  test('Profile page loads successfully', async ({ page }) => {
    await page.goto('/profile');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on profile page
    expect(page.url()).toContain('/profile');
    
    // Check for specific profile content
    const hasProfileHeading = await page.locator('h2:has-text("Your Profile")').count() > 0;
    const hasComingSoon = await page.locator('text=Profile management coming soon').count() > 0;
    
    expect(hasProfileHeading || hasComingSoon).toBeTruthy();
  });

  test('Notifications page loads successfully', async ({ page }) => {
    await page.goto('/notifications');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on notifications page
    expect(page.url()).toContain('/notifications');
    
    // Check for specific notifications content
    const hasNotificationsHeading = await page.locator('h2:has-text("Notifications")').count() > 0;
    const hasComingSoon = await page.locator('text=Notification system coming soon').count() > 0;
    
    expect(hasNotificationsHeading || hasComingSoon).toBeTruthy();
  });

  test('Availability page loads successfully', async ({ page }) => {
    await page.goto('/availability');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on availability page
    expect(page.url()).toContain('/availability');
    
    // Check for specific availability content
    const hasAvailabilityHeading = await page.locator('h2:has-text("Availability Management")').count() > 0;
    const hasComingSoon = await page.locator('text=Availability features coming soon').count() > 0;
    
    expect(hasAvailabilityHeading || hasComingSoon).toBeTruthy();
  });

  test('Admin page loads successfully', async ({ page }) => {
    await page.goto('/admin');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on admin page
    expect(page.url()).toContain('/admin');
    
    // Check for specific admin content
    const hasAdminHeading = await page.locator('h2:has-text("Admin Dashboard")').count() > 0;
    const hasComingSoon = await page.locator('text=Admin features coming soon').count() > 0;
    
    expect(hasAdminHeading || hasComingSoon).toBeTruthy();
  });

  test('Connect page loads successfully', async ({ page }) => {
    await page.goto('/connect');
    
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000); // Give JS routing time
    
    // Should be on connect page
    expect(page.url()).toContain('/connect');
    
    // Check for specific connect content
    const hasConnectHeading = await page.locator('h2:has-text("Lo.Co Connect")').count() > 0;
    const hasComingSoon = await page.locator('text=Connect features coming soon').count() > 0;
    
    expect(hasConnectHeading || hasComingSoon).toBeTruthy();
  });
});