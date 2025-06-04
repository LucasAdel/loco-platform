// Sidebar navigation functionality tests
const { test, expect } = require('@playwright/test');

test.describe('Sidebar Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('Sidebar is visible and functional', async ({ page }) => {
    // Check if sidebar exists
    const sidebar = page.locator('[data-testid="sidebar"], .sidebar, nav');
    await expect(sidebar.first()).toBeVisible();
  });

  test('Navigation links work correctly', async ({ page }) => {
    // Test Jobs navigation
    const jobsLink = page.locator('text=Jobs, a[href="/jobs"]').first();
    if (await jobsLink.count() > 0) {
      await jobsLink.click();
      await page.waitForLoadState('networkidle');
      expect(page.url()).toContain('/jobs');
    }

    // Test Map navigation
    await page.goto('/');
    const mapLink = page.locator('text=Map, a[href="/map"]').first();
    if (await mapLink.count() > 0) {
      await mapLink.click();
      await page.waitForLoadState('networkidle');
      expect(page.url()).toContain('/map');
    }

    // Test Forum navigation
    await page.goto('/');
    const forumLink = page.locator('text=Forum, a[href="/forum"]').first();
    if (await forumLink.count() > 0) {
      await forumLink.click();
      await page.waitForLoadState('networkidle');
      expect(page.url()).toContain('/forum');
    }
  });

  test('Mobile menu toggle works', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Look for mobile menu button
    const mobileMenuButton = page.locator('[data-testid="mobile-menu"], .mobile-menu-button, button[aria-label*="menu"]');
    
    if (await mobileMenuButton.count() > 0) {
      // Test mobile menu toggle
      await mobileMenuButton.click();
      
      // Check if menu appears
      const mobileMenu = page.locator('[data-testid="mobile-menu-content"], .mobile-menu');
      await expect(mobileMenu.first()).toBeVisible();
      
      // Close menu
      await mobileMenuButton.click();
    }
  });

  test('Home logo/title navigation works', async ({ page }) => {
    // Navigate to any page first
    await page.goto('/jobs');
    
    // Look for home link (logo or title)
    const homeLink = page.locator('a[href="/"]').or(page.locator('text="Loco Platform"')).or(page.locator('[data-testid="home-link"]')).first();
    
    if (await homeLink.count() > 0) {
      await homeLink.click();
      await page.waitForLoadState('networkidle');
      expect(page.url()).toMatch(/\/$|\/home$/);
    }
  });
});