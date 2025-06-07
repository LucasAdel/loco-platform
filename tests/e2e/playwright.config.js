// @ts-check
const { defineConfig, devices } = require('@playwright/test');

/**
 * @see https://playwright.dev/docs/test-configuration
 */
module.exports = defineConfig({
  testDir: './specs',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  
  use: {
    baseURL: 'http://localhost:3080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },

  projects: [
    {
      name: 'chromium',
      use: { 
        ...devices['Desktop Chrome'],
        headless: true,
      },
    },
    {
      name: 'mobile',
      use: { 
        ...devices['iPhone 12'],
        headless: true,
      },
    },
    {
      name: 'tablet',
      use: {
        ...devices['iPad Pro'],
        headless: true,
      },
    },
  ],

  webServer: {
    command: 'cd ../../static && python3 -m http.server 3080',
    port: 3080,
    reuseExistingServer: !process.env.CI,
  },
});