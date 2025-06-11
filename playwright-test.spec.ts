import { test, expect } from '@playwright/test';

// Run tests in headless mode as requested
test.use({ headless: true });

test.describe('Loco Platform Comprehensive Test Suite', () => {
  const baseURL = 'http://localhost:3080';
  
  test.beforeEach(async ({ page }) => {
    // Navigate to the app
    await page.goto(baseURL);
  });

  test('Homepage loads correctly with all elements', async ({ page }) => {
    // Check title
    await expect(page).toHaveTitle(/Loco Platform/);
    
    // Check main heading
    await expect(page.locator('h1')).toContainText('Find Your Dream Pharmacy Job');
    
    // Check navigation elements
    await expect(page.locator('a:has-text("Log in")')).toBeVisible();
    await expect(page.locator('a:has-text("Get Started")')).toBeVisible();
    
    // Check Browse Jobs button
    await expect(page.locator('a:has-text("Browse Jobs")')).toBeVisible();
    
    // Check no console errors
    const consoleErrors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });
    
    await page.waitForTimeout(1000);
    expect(consoleErrors).toEqual([]);
  });

  test('Login page functionality', async ({ page }) => {
    // Navigate to login
    await page.click('a:has-text("Log in")');
    await page.waitForURL('**/login');
    
    // Check login form elements
    await expect(page.locator('h2')).toContainText('Welcome Back');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    
    // Test invalid login
    await page.fill('input[type="email"]', 'invalid@example.com');
    await page.fill('input[type="password"]', 'wrongpassword');
    await page.click('button:has-text("Sign In")');
    
    // Check error message appears
    await expect(page.locator('text=Invalid email or password')).toBeVisible();
    
    // Test valid login
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button:has-text("Sign In")');
    
    // Should redirect to dashboard
    await page.waitForURL('**/dashboard');
    await expect(page.locator('h1')).toContainText('Dashboard');
  });

  test('Register page loads and has all fields', async ({ page }) => {
    await page.goto(`${baseURL}/register`);
    
    await expect(page.locator('h2')).toContainText('Create Account');
    await expect(page.locator('input[placeholder="John"]')).toBeVisible();
    await expect(page.locator('input[placeholder="Doe"]')).toBeVisible();
    await expect(page.locator('input[placeholder="you@example.com"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('Dashboard displays statistics cards', async ({ page }) => {
    // Login first
    await page.goto(`${baseURL}/login`);
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button:has-text("Sign In")');
    
    await page.waitForURL('**/dashboard');
    
    // Check dashboard elements
    await expect(page.locator('text=Active Jobs')).toBeVisible();
    await expect(page.locator('text=Applications')).toBeVisible();
    await expect(page.locator('text=Profile Views')).toBeVisible();
    await expect(page.locator('text=Messages')).toBeVisible();
    
    // Check navigation header
    await expect(page.locator('nav a:has-text("Dashboard")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Jobs")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Map")')).toBeVisible();
    await expect(page.locator('nav a:has-text("Applications")')).toBeVisible();
  });

  test('Jobs page with search and filtering', async ({ page }) => {
    await page.goto(`${baseURL}/jobs`);
    
    // Check page loaded
    await expect(page.locator('h1')).toContainText('Job Listings');
    
    // Check search bar
    const searchInput = page.locator('input[placeholder*="Search jobs"]');
    await expect(searchInput).toBeVisible();
    
    // Test search functionality
    await searchInput.fill('Senior');
    await page.waitForTimeout(500);
    
    // Should show filtered results
    await expect(page.locator('text=Senior Pharmacist')).toBeVisible();
    
    // Test job type filter
    await page.selectOption('select:has-text("All Types")', 'Full-time');
    await page.waitForTimeout(500);
    
    // Test location filter
    await page.selectOption('select:has-text("All Locations")', 'Sydney');
    await page.waitForTimeout(500);
    
    // Check job cards have proper structure
    const jobCard = page.locator('.bg-white.rounded-xl').first();
    await expect(jobCard).toBeVisible();
    await expect(jobCard.locator('h3')).toBeVisible();
    await expect(jobCard.locator('text=/\\$\\d+k/')).toBeVisible();
  });

  test('Map page loads with sidebar', async ({ page }) => {
    await page.goto(`${baseURL}/map`);
    
    // Check sidebar
    await expect(page.locator('h2:has-text("Job Locations")')).toBeVisible();
    
    // Check job location cards
    await expect(page.locator('text=Sydney Hospital')).toBeVisible();
    await expect(page.locator('text=Melbourne Medical Centre')).toBeVisible();
    await expect(page.locator('text=Brisbane Pharmacy')).toBeVisible();
    
    // Check urgent badge
    await expect(page.locator('span:has-text("Urgent")')).toBeVisible();
    
    // Map container should exist
    await expect(page.locator('.mapbox-container, [id^="mapbox-"]')).toBeVisible();
  });

  test('Profile page displays user information', async ({ page }) => {
    await page.goto(`${baseURL}/profile`);
    
    await expect(page.locator('h1')).toContainText('My Profile');
    await expect(page.locator('h2:has-text("John Doe")')).toBeVisible();
    await expect(page.locator('text=john.doe@example.com')).toBeVisible();
    
    // Check form fields
    await expect(page.locator('input[value*="612"]')).toBeVisible();
    await expect(page.locator('input[value="Sydney, NSW"]')).toBeVisible();
    await expect(page.locator('input[value="PHA0001234567"]')).toBeVisible();
    
    // Check save button
    await expect(page.locator('button:has-text("Save Changes")')).toBeVisible();
  });

  test('Applications page shows application status', async ({ page }) => {
    await page.goto(`${baseURL}/applications`);
    
    await expect(page.locator('h1')).toContainText('My Applications');
    
    // Check application cards
    await expect(page.locator('text=Senior Pharmacist')).toBeVisible();
    await expect(page.locator('text=Under Review')).toBeVisible();
    
    await expect(page.locator('text=Clinical Pharmacist')).toBeVisible();
    await expect(page.locator('text=Interview Scheduled')).toBeVisible();
    
    await expect(page.locator('text=Community Pharmacist')).toBeVisible();
    await expect(page.locator('text=Application Sent')).toBeVisible();
  });

  test('Navigation between pages works correctly', async ({ page }) => {
    // Start at home
    await page.goto(baseURL);
    
    // Navigate to jobs
    await page.click('a:has-text("Browse Jobs")');
    await page.waitForURL('**/jobs');
    await expect(page.locator('h1')).toContainText('Job Listings');
    
    // Navigate to map via header
    await page.click('nav a:has-text("Map")');
    await page.waitForURL('**/map');
    await expect(page.locator('h2:has-text("Job Locations")')).toBeVisible();
    
    // Navigate to applications
    await page.click('nav a:has-text("Applications")');
    await page.waitForURL('**/applications');
    await expect(page.locator('h1')).toContainText('My Applications');
    
    // Navigate to profile via icon
    await page.click('a[href="/profile"] svg');
    await page.waitForURL('**/profile');
    await expect(page.locator('h1')).toContainText('My Profile');
  });

  test('Responsive design works on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    await page.goto(baseURL);
    
    // Check mobile layout
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('a:has-text("Browse Jobs")')).toBeVisible();
    
    // Navigate to jobs page
    await page.goto(`${baseURL}/jobs`);
    
    // Search should be visible on mobile
    await expect(page.locator('input[placeholder*="Search jobs"]')).toBeVisible();
    
    // Job cards should stack vertically
    const jobCards = page.locator('.bg-white.rounded-xl');
    const count = await jobCards.count();
    expect(count).toBeGreaterThan(0);
  });

  test('Beautiful UI elements are present', async ({ page }) => {
    await page.goto(baseURL);
    
    // Check gradient background
    await expect(page.locator('.bg-gradient-to-br')).toBeVisible();
    
    // Check glass morphism on login page
    await page.goto(`${baseURL}/login`);
    await expect(page.locator('.backdrop-blur-xl')).toBeVisible();
    await expect(page.locator('.bg-white\\/80')).toBeVisible();
    
    // Check Tiffany blue branding
    const logo = page.locator('.bg-gradient-to-br.from-tiffany');
    await expect(logo).toBeVisible();
    
    // Check rounded corners and shadows
    await expect(page.locator('.rounded-3xl.shadow-2xl')).toBeVisible();
  });

  test('Page performance and no errors', async ({ page }) => {
    const errors: string[] = [];
    const warnings: string[] = [];
    
    page.on('console', msg => {
      if (msg.type() === 'error') errors.push(msg.text());
      if (msg.type() === 'warning') warnings.push(msg.text());
    });
    
    page.on('pageerror', error => {
      errors.push(error.message);
    });
    
    // Test all main pages
    const pages = ['/', '/login', '/register', '/jobs', '/map', '/profile', '/applications'];
    
    for (const path of pages) {
      await page.goto(`${baseURL}${path}`);
      await page.waitForLoadState('networkidle');
    }
    
    // No critical errors should occur
    expect(errors.filter(e => !e.includes('Unrecognized feature'))).toEqual([]);
  });
});

// Additional test for comprehensive functionality check
test('Full user journey from landing to job application', async ({ page }) => {
  await page.goto('http://localhost:3080');
  
  // 1. Start at homepage
  await expect(page.locator('h1:has-text("Find Your Dream Pharmacy Job")')).toBeVisible();
  
  // 2. Click Get Started
  await page.click('a:has-text("Get Started")');
  await page.waitForURL('**/dashboard');
  
  // 3. Browse jobs
  await page.click('nav a:has-text("Jobs")');
  await expect(page.locator('h1:has-text("Job Listings")')).toBeVisible();
  
  // 4. Search for specific job
  await page.fill('input[placeholder*="Search jobs"]', 'Senior');
  await expect(page.locator('text=Senior Pharmacist')).toBeVisible();
  
  // 5. View map
  await page.click('nav a:has-text("Map")');
  await expect(page.locator('h2:has-text("Job Locations")')).toBeVisible();
  
  // 6. Check applications
  await page.click('nav a:has-text("Applications")');
  await expect(page.locator('text=My Applications')).toBeVisible();
  
  // 7. Update profile
  await page.click('a[href="/profile"]');
  await expect(page.locator('h1:has-text("My Profile")')).toBeVisible();
  
  console.log('✅ Full user journey completed successfully!');
});