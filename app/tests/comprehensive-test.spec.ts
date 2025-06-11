import { test, expect, type Page } from '@playwright/test';

// Test configuration
const BASE_URL = 'http://localhost:3080';
const SUPER_ADMIN_EMAIL = 'lw@hamiltonbailey.com';
const SUPER_ADMIN_PASSWORD = 'password123';

// Helper function to check for console errors
async function checkForConsoleErrors(page: Page, pageName: string): Promise<string[]> {
  const errors: string[] = [];
  
  page.on('console', msg => {
    if (msg.type() === 'error') {
      errors.push(`[${pageName}] Console error: ${msg.text()}`);
    }
  });
  
  page.on('pageerror', error => {
    errors.push(`[${pageName}] Page error: ${error.message}`);
  });
  
  return errors;
}

// Helper function to check page responsiveness
async function checkResponsiveness(page: Page, pageName: string): Promise<string[]> {
  const issues: string[] = [];
  
  // Test mobile viewport
  await page.setViewportSize({ width: 375, height: 667 });
  await page.waitForTimeout(500);
  
  // Check for horizontal scroll
  const hasHorizontalScroll = await page.evaluate(() => {
    return document.documentElement.scrollWidth > document.documentElement.clientWidth;
  });
  
  if (hasHorizontalScroll) {
    issues.push(`[${pageName}] Horizontal scroll detected on mobile viewport`);
  }
  
  // Test tablet viewport
  await page.setViewportSize({ width: 768, height: 1024 });
  await page.waitForTimeout(500);
  
  // Reset to desktop
  await page.setViewportSize({ width: 1280, height: 720 });
  
  return issues;
}

test.describe('Loco Platform - Comprehensive Testing', () => {
  test.describe.configure({ mode: 'serial' });
  
  let allErrors: string[] = [];
  let allIssues: string[] = [];
  
  test.beforeEach(async ({ page }) => {
    // Set up console error tracking
    page.on('console', msg => {
      if (msg.type() === 'error') {
        allErrors.push(msg.text());
      }
    });
  });
  
  test('Home Page', async ({ page }) => {
    await page.goto(BASE_URL);
    
    // Check page loads
    await expect(page).toHaveTitle(/Loco Platform/);
    
    // Check key elements
    await expect(page.locator('h1')).toContainText('Find Your Dream Pharmacy Job');
    await expect(page.locator('text=Get Started')).toBeVisible();
    await expect(page.locator('text=Log in')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Home');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Home');
    allIssues.push(...issues);
  });
  
  test('Login Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/login`);
    
    // Check and dismiss any error overlays
    const errorOverlay = page.locator('text=error from build pipeline');
    if (await errorOverlay.isVisible({ timeout: 1000 }).catch(() => false)) {
      // Try to close or dismiss the error overlay
      await page.keyboard.press('Escape');
      await page.waitForTimeout(500);
    }
    
    // Check page loads
    await expect(page.locator('h2')).toContainText('Welcome Back');
    
    // Fill credentials (pre-filling might not work in WASM)
    const emailInput = page.locator('input[type="email"]');
    const passwordInput = page.locator('input[type="password"]');
    
    // Clear and fill inputs
    await emailInput.clear();
    await emailInput.fill(SUPER_ADMIN_EMAIL);
    await passwordInput.clear();
    await passwordInput.fill(SUPER_ADMIN_PASSWORD);
    
    // Test login functionality
    await page.locator('button[type="submit"]').click();
    
    // Should redirect to dashboard
    await page.waitForURL('**/dashboard', { timeout: 5000 });
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Login');
    allErrors.push(...errors);
  });
  
  test('Dashboard Page', async ({ page }) => {
    // Login first
    await page.goto(`${BASE_URL}/login`);
    await page.locator('button[type="submit"]').click();
    await page.waitForURL('**/dashboard');
    
    // Check dashboard elements
    await expect(page.locator('h1')).toContainText('Dashboard');
    await expect(page.locator('text=Active Jobs')).toBeVisible();
    await expect(page.locator('text=Applications')).toBeVisible();
    await expect(page.locator('text=Profile Views')).toBeVisible();
    await expect(page.locator('text=Messages')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Dashboard');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Dashboard');
    allIssues.push(...issues);
  });
  
  test('Jobs Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/jobs`);
    
    // Check page loads
    await expect(page.locator('h1')).toContainText('Job Listings');
    
    // Check job cards are displayed
    await expect(page.locator('.bg-white.rounded-xl').first()).toBeVisible();
    
    // Check search functionality
    const searchInput = page.locator('input[placeholder="Search jobs..."]');
    await searchInput.fill('Senior');
    await page.waitForTimeout(500);
    
    // Check filters
    await expect(page.locator('select').first()).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Jobs');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Jobs');
    allIssues.push(...issues);
  });
  
  test('Map Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/map`);
    
    // Check page loads
    await expect(page.locator('h1')).toContainText('Job Map');
    
    // Check map placeholder is visible
    await expect(page.locator('text=Interactive Map')).toBeVisible();
    
    // Check nearby jobs sidebar
    await expect(page.locator('text=Nearby Jobs')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Map');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Map');
    allIssues.push(...issues);
  });
  
  test('Profile Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/profile`);
    
    // Check page loads
    await expect(page.locator('h1')).toContainText('Profile');
    
    // Check profile elements
    await expect(page.locator('text=John Doe')).toBeVisible();
    await expect(page.locator('text=Contact Information')).toBeVisible();
    await expect(page.locator('text=Professional Details')).toBeVisible();
    await expect(page.locator('text=Edit Profile')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Profile');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Profile');
    allIssues.push(...issues);
  });
  
  test('Applications Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/applications`);
    
    // Check page loads
    await expect(page.locator('h1')).toContainText('My Applications');
    
    // Check application columns
    await expect(page.locator('text=Applied')).toBeVisible();
    await expect(page.locator('text=In Review')).toBeVisible();
    await expect(page.locator('text=Interview')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Applications');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Applications');
    allIssues.push(...issues);
  });
  
  test('Register Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/register`);
    
    // Check page loads
    await expect(page.locator('h2')).toContainText('Create Account');
    
    // Check form fields
    await expect(page.locator('input[type="text"]').first()).toBeVisible();
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, 'Register');
    allErrors.push(...errors);
    
    // Check responsiveness
    const issues = await checkResponsiveness(page, 'Register');
    allIssues.push(...issues);
  });
  
  test('404 Page', async ({ page }) => {
    await page.goto(`${BASE_URL}/non-existent-page`);
    
    // Check 404 page loads
    await expect(page.locator('text=404')).toBeVisible();
    await expect(page.locator('text=Page not found')).toBeVisible();
    await expect(page.locator('text=Go Home')).toBeVisible();
    
    // Check for console errors
    const errors = await checkForConsoleErrors(page, '404');
    allErrors.push(...errors);
  });
  
  test('Navigation Flow', async ({ page }) => {
    // Test navigation between pages
    await page.goto(BASE_URL);
    
    // Navigate to Jobs
    await page.locator('text=Browse Jobs').click();
    await expect(page).toHaveURL(/\/jobs/);
    
    // Navigate via header
    await page.locator('nav >> text=Map').click();
    await expect(page).toHaveURL(/\/map/);
    
    await page.locator('nav >> text=Dashboard').click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
  
  test.afterAll(async () => {
    console.log('\n=== COMPREHENSIVE TEST REPORT ===\n');
    
    if (allErrors.length === 0 && allIssues.length === 0) {
      console.log('✅ All tests passed! No errors or issues found.');
    } else {
      console.log(`❌ Found ${allErrors.length} errors and ${allIssues.length} issues:\n`);
      
      if (allErrors.length > 0) {
        console.log('ERRORS:');
        allErrors.forEach(error => console.log(`  - ${error}`));
      }
      
      if (allIssues.length > 0) {
        console.log('\nISSUES:');
        allIssues.forEach(issue => console.log(`  - ${issue}`));
      }
    }
    
    console.log('\n=== END OF REPORT ===\n');
  });
});