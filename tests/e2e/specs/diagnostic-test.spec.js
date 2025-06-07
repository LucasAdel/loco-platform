// @ts-check
const { test, expect } = require('@playwright/test');

const PAGES_TO_DIAGNOSE = [
  '/jobs.html',
  '/job-detail.html',
  '/map.html',
  '/login.html',
  '/profile.html',
  '/dashboard.html'
];

test.describe('Diagnostic Tests', () => {
  for (const page of PAGES_TO_DIAGNOSE) {
    test(`diagnose ${page}`, async ({ page: browserPage }) => {
      const errors = [];
      const warnings = [];
      
      // Capture console messages
      browserPage.on('console', msg => {
        if (msg.type() === 'error') {
          errors.push({
            type: 'console-error',
            text: msg.text(),
            location: msg.location()
          });
        } else if (msg.type() === 'warning') {
          warnings.push({
            type: 'console-warning',
            text: msg.text()
          });
        }
      });
      
      // Capture page errors
      browserPage.on('pageerror', err => {
        errors.push({
          type: 'page-error',
          message: err.message,
          stack: err.stack
        });
      });
      
      // Capture failed requests
      browserPage.on('requestfailed', request => {
        errors.push({
          type: 'request-failed',
          url: request.url(),
          failure: request.failure()
        });
      });
      
      try {
        const response = await browserPage.goto(page);
        console.log(`\n=== ${page} ===`);
        console.log(`Status: ${response.status()}`);
        
        if (errors.length > 0) {
          console.log('\nErrors:');
          errors.forEach((err, i) => {
            console.log(`${i + 1}. ${err.type}: ${err.text || err.message || err.url}`);
            if (err.location) {
              console.log(`   Location: ${err.location.url}:${err.location.lineNumber}`);
            }
          });
        }
        
        if (warnings.length > 0) {
          console.log('\nWarnings:');
          warnings.forEach((warn, i) => {
            console.log(`${i + 1}. ${warn.text}`);
          });
        }
        
        // Check for missing stylesheets
        const missingStyles = await browserPage.evaluate(() => {
          const links = document.querySelectorAll('link[rel="stylesheet"]');
          const missing = [];
          links.forEach(link => {
            const sheet = link.sheet;
            if (!sheet || sheet.cssRules.length === 0) {
              missing.push(link.href);
            }
          });
          return missing;
        });
        
        if (missingStyles.length > 0) {
          console.log('\nMissing stylesheets:');
          missingStyles.forEach(style => console.log(`- ${style}`));
        }
        
      } catch (err) {
        console.log(`Failed to load ${page}: ${err.message}`);
      }
    });
  }
});