const puppeteer = require('puppeteer');

async function testPages() {
    const browser = await puppeteer.launch({ headless: true });
    const page = await browser.newPage();
    
    const baseURL = 'http://localhost:3080';
    const pages = [
        { path: '/', name: 'Homepage' },
        { path: '/login', name: 'Login' },
        { path: '/dashboard', name: 'Dashboard' },
        { path: '/jobs', name: 'Jobs' },
    ];
    
    console.log('Testing Loco Platform Pages...\n');
    
    for (const pageInfo of pages) {
        try {
            const response = await page.goto(`${baseURL}${pageInfo.path}`, { 
                waitUntil: 'networkidle2',
                timeout: 10000 
            });
            
            const status = response.status();
            const title = await page.title();
            const hasContent = await page.evaluate(() => document.body.innerText.length > 0);
            
            console.log(`✅ ${pageInfo.name}:`);
            console.log(`   Status: ${status}`);
            console.log(`   Title: ${title}`);
            console.log(`   Has Content: ${hasContent}`);
            
            // Check for errors
            const errors = await page.evaluate(() => {
                const errorElements = document.querySelectorAll('.error, [class*="error"]');
                return Array.from(errorElements).map(el => el.textContent).filter(text => text.length > 0);
            });
            
            if (errors.length > 0) {
                console.log(`   ⚠️  Errors found: ${errors.join(', ')}`);
            }
            
        } catch (error) {
            console.log(`❌ ${pageInfo.name}: Failed to load`);
            console.log(`   Error: ${error.message}`);
        }
        console.log('');
    }
    
    await browser.close();
}

testPages().catch(console.error);