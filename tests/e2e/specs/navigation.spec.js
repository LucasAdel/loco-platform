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
    
    // Should be on forum page
    expect(page.url()).toContain('/forum');
    
    // Check for forum content (even if placeholder)
    const hasForumContent = await page.locator('text=/forum/i, text=/discussion/i, text=/coming soon/i').count() > 0;
    expect(hasForumContent).toBeTruthy();
  });

  test('Profile page loads successfully', async ({ page }) => {
    await page.goto('/profile');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on profile page
    expect(page.url()).toContain('/profile');
    
    // Check for profile content (even if placeholder)
    const hasProfileContent = await page.locator('text=/profile/i, text=/account/i, text=/coming soon/i').count() > 0;
    expect(hasProfileContent).toBeTruthy();
  });

  test('Notifications page loads successfully', async ({ page }) => {
    await page.goto('/notifications');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on notifications page
    expect(page.url()).toContain('/notifications');
    
    // Check for notifications content (even if placeholder)
    const hasNotificationsContent = await page.locator('text=/notification/i, text=/alert/i, text=/coming soon/i').count() > 0;
    expect(hasNotificationsContent).toBeTruthy();
  });

  test('Availability page loads successfully', async ({ page }) => {
    await page.goto('/availability');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on availability page
    expect(page.url()).toContain('/availability');
    
    // Check for availability content (even if placeholder)
    const hasAvailabilityContent = await page.locator('text=/availability/i, text=/schedule/i, text=/coming soon/i').count() > 0;
    expect(hasAvailabilityContent).toBeTruthy();
  });

  test('Admin page loads successfully', async ({ page }) => {
    await page.goto('/admin');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on admin page
    expect(page.url()).toContain('/admin');
    
    // Check for admin content (even if placeholder)
    const hasAdminContent = await page.locator('text=/admin/i, text=/dashboard/i, text=/coming soon/i').count() > 0;
    expect(hasAdminContent).toBeTruthy();
  });

  test('Connect page loads successfully', async ({ page }) => {
    await page.goto('/connect');
    
    await page.waitForLoadState('networkidle');
    
    // Should be on connect page
    expect(page.url()).toContain('/connect');
    
    // Check for connect content (even if placeholder)
    const hasConnectContent = await page.locator('text=/connect/i, text=/network/i, text=/coming soon/i').count() > 0;
    expect(hasConnectContent).toBeTruthy();
  });
});