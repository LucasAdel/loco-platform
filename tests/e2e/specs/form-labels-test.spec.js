// @ts-check
const { test, expect } = require('@playwright/test');

test('Check form labels on Login page', async ({ page }) => {
    await page.goto('/login.html');
    await page.waitForLoadState('networkidle');
    
    // Get all form inputs that need labels
    const inputsInfo = await page.evaluate(() => {
        const inputs = document.querySelectorAll('input:not([type="submit"]):not([type="button"]), textarea, select');
        const info = [];
        
        inputs.forEach(input => {
            const id = input.id;
            const name = input.name;
            const type = input.type;
            const placeholder = input.placeholder;
            const ariaLabel = input.getAttribute('aria-label');
            const label = id ? document.querySelector(`label[for="${id}"]`) : null;
            
            info.push({
                tag: input.tagName,
                id: id || 'none',
                name: name || 'none',
                type: type || 'none',
                placeholder: placeholder || 'none',
                hasLabel: !!label,
                hasAriaLabel: !!ariaLabel,
                labelText: label ? label.textContent.trim() : 'none'
            });
        });
        
        return info;
    });
    
    console.log('Form inputs found:', inputsInfo);
    
    // Check each input
    inputsInfo.forEach(input => {
        if (!input.hasLabel && !input.hasAriaLabel) {
            console.log(`Missing label for: ${input.tag} (id: ${input.id}, name: ${input.name}, type: ${input.type})`);
        }
    });
});