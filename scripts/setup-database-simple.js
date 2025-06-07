#!/usr/bin/env node

/**
 * Simple database setup for Supabase
 * Creates essential tables for the Loco Platform
 */

import { createClient } from '@supabase/supabase-js';

// Supabase configuration
const SUPABASE_URL = 'https://kpmmsogskffsiubbegvc.supabase.co';
const SUPABASE_ANON_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImtwbW1zb2dza2Zmc2l1YmJlZ3ZjIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MzU5Nzk1NDUsImV4cCI6MjA1MTU1NTU0NX0.IqMGLpkXLf7QPFMH6kj6RFqoQQPnxqiJzJdZPgQ7tPU';

// ANSI colors
const colors = {
    reset: '\x1b[0m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    red: '\x1b[31m',
    blue: '\x1b[34m',
    cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

async function setupDatabase() {
    log('\n🚀 Setting up Loco Platform Database\n', 'cyan');
    
    try {
        // Initialize Supabase client
        const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);
        
        // Test connection
        log('🔍 Testing connection...', 'blue');
        const { data: test, error: testError } = await supabase.auth.getSession();
        
        if (testError) {
            log(`✗ Connection failed: ${testError.message}`, 'red');
            return;
        }
        
        log('✓ Connected to Supabase successfully!', 'green');
        
        // Check if tables exist
        log('\n📊 Checking existing tables...', 'blue');
        
        const tables = [
            'user_profiles',
            'jobs', 
            'applications',
            'saved_jobs',
            'notifications'
        ];
        
        for (const table of tables) {
            const { count, error } = await supabase
                .from(table)
                .select('*', { count: 'exact', head: true });
                
            if (error && error.code === 'PGRST116') {
                log(`  ✗ Table '${table}' does not exist`, 'yellow');
            } else if (error) {
                log(`  ⚠️  Error checking '${table}': ${error.message}`, 'yellow');
            } else {
                log(`  ✓ Table '${table}' exists (${count || 0} rows)`, 'green');
            }
        }
        
        log('\n📝 Database Setup Instructions:', 'cyan');
        log('\n1. Go to your Supabase Dashboard:', 'yellow');
        log(`   ${SUPABASE_URL}`, 'blue');
        
        log('\n2. Navigate to SQL Editor', 'yellow');
        
        log('\n3. Create a new query and paste the contents of:', 'yellow');
        log('   supabase/migrations/20250605000003_complete_schema.sql', 'blue');
        
        log('\n4. Click "RUN" to execute the schema', 'yellow');
        
        log('\n5. After running, you should see all tables created', 'yellow');
        
        log('\n💡 Alternative: Use Supabase CLI', 'cyan');
        log('   npx supabase db push', 'blue');
        
        // Create some test data
        log('\n🧪 Attempting to create test user...', 'cyan');
        
        const { data: authData, error: authError } = await supabase.auth.signUp({
            email: 'test@example.com',
            password: 'testpassword123',
            options: {
                data: {
                    first_name: 'Test',
                    last_name: 'User'
                }
            }
        });
        
        if (authError) {
            log(`  ⚠️  Could not create test user: ${authError.message}`, 'yellow');
        } else {
            log('  ✓ Test user created successfully!', 'green');
        }
        
    } catch (error) {
        log(`\n❌ Error: ${error.message}`, 'red');
    }
}

// Run the setup
setupDatabase();