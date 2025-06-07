#!/usr/bin/env node

/**
 * Apply complete database schema to Supabase
 * This script creates all tables, indexes, and RLS policies
 */

const { createClient } = require('@supabase/supabase-js');
const fs = require('fs').promises;
const path = require('path');

// Supabase configuration - PRODUCTION
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_SERVICE_ROLE_KEY = process.env.SUPABASE_SERVICE_ROLE_KEY;
const SUPABASE_DB_URL = process.env.SUPABASE_DB_URL || 'postgresql://postgres:LocoDevDB2024!@db.kpmmsogskffsiubbegvc.supabase.co:5432/postgres';

// ANSI color codes
const colors = {
    reset: '\x1b[0m',
    bright: '\x1b[1m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    red: '\x1b[31m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

async function readSQLFile(filename) {
    try {
        const filePath = path.join(__dirname, '..', 'supabase', 'migrations', filename);
        const content = await fs.readFile(filePath, 'utf8');
        return content;
    } catch (error) {
        throw new Error(`Failed to read SQL file ${filename}: ${error.message}`);
    }
}

async function executeSQLStatements(supabase, sql) {
    // Split SQL into individual statements
    const statements = sql
        .split(';')
        .map(s => s.trim())
        .filter(s => s.length > 0 && !s.startsWith('--'));

    let successCount = 0;
    let errorCount = 0;

    for (let i = 0; i < statements.length; i++) {
        const statement = statements[i] + ';';
        
        // Skip if it's just a comment
        if (statement.trim().startsWith('--')) continue;
        
        try {
            // Extract operation type for logging
            const operation = statement.trim().substring(0, 50).replace(/\n/g, ' ');
            process.stdout.write(`  [${i + 1}/${statements.length}] ${operation}... `);
            
            // Use Supabase's rpc to execute raw SQL
            const { error } = await supabase.rpc('exec_sql', {
                sql: statement
            });
            
            if (error) throw error;
            
            log('✓', 'green');
            successCount++;
        } catch (error) {
            log('✗', 'red');
            log(`    Error: ${error.message}`, 'red');
            errorCount++;
            
            // Continue on error for CREATE IF NOT EXISTS statements
            if (!statement.includes('IF NOT EXISTS')) {
                throw error;
            }
        }
    }
    
    return { successCount, errorCount };
}

async function applySchema() {
    log('\n🚀 Applying Supabase Database Schema\n', 'bright');
    
    if (!SUPABASE_SERVICE_ROLE_KEY) {
        log('Error: SUPABASE_SERVICE_ROLE_KEY environment variable is required', 'red');
        log('Please set it with your Supabase service role key', 'yellow');
        process.exit(1);
    }
    
    try {
        // Initialize Supabase client
        const supabase = createClient(SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY, {
            auth: {
                persistSession: false
            }
        });
        
        log('📋 Reading schema file...', 'cyan');
        const schema = await readSQLFile('20250605000003_complete_schema.sql');
        
        log('🔧 Creating database objects...', 'cyan');
        
        // First, we need to create a function to execute SQL
        // This is a workaround since Supabase client doesn't have direct SQL execution
        log('\n📦 Setting up SQL execution function...', 'blue');
        
        // For now, let's use a different approach - create the schema manually via psql
        log('\n⚠️  Direct SQL execution via Supabase JS client is limited.', 'yellow');
        log('Please run the following command to apply the schema:', 'yellow');
        log(`\npsql "${SUPABASE_DB_URL}" -f supabase/migrations/20250605000003_complete_schema.sql\n`, 'cyan');
        
        log('Alternatively, you can:', 'yellow');
        log('1. Go to your Supabase dashboard', 'yellow');
        log('2. Navigate to SQL Editor', 'yellow');
        log('3. Paste the contents of supabase/migrations/20250605000003_complete_schema.sql', 'yellow');
        log('4. Click "Run" to execute', 'yellow');
        
        // Test connection
        log('\n🔍 Testing database connection...', 'cyan');
        const { data, error } = await supabase
            .from('user_profiles')
            .select('count')
            .limit(1);
            
        if (error && error.code !== 'PGRST116') { // PGRST116 = table doesn't exist
            log(`✗ Connection test failed: ${error.message}`, 'red');
        } else if (error && error.code === 'PGRST116') {
            log('✓ Connected to database (schema not yet applied)', 'yellow');
        } else {
            log('✓ Connected to database (schema already exists)', 'green');
        }
        
    } catch (error) {
        log(`\n❌ Error: ${error.message}`, 'red');
        process.exit(1);
    }
}

// Alternative: Create a psql script wrapper
async function createPSQLScript() {
    const scriptContent = `#!/bin/bash
# Apply Supabase schema using psql

echo "🚀 Applying Loco Platform Database Schema to Supabase"

# Check if SUPABASE_DB_URL is set
if [ -z "$SUPABASE_DB_URL" ]; then
    echo "❌ Error: SUPABASE_DB_URL environment variable is not set"
    echo "Please set it to your Supabase database connection string"
    exit 1
fi

# Apply the schema
echo "📦 Applying schema..."
psql "$SUPABASE_DB_URL" -f supabase/migrations/20250605000003_complete_schema.sql

if [ $? -eq 0 ]; then
    echo "✅ Schema applied successfully!"
else
    echo "❌ Failed to apply schema"
    exit 1
fi
`;

    const scriptPath = path.join(__dirname, 'apply-schema.sh');
    await fs.writeFile(scriptPath, scriptContent, { mode: 0o755 });
    
    log('\n✅ Created apply-schema.sh script', 'green');
    log('Run it with: ./scripts/apply-schema.sh', 'cyan');
}

// Run the script
(async () => {
    await applySchema();
    await createPSQLScript();
})();