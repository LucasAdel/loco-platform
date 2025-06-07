#!/usr/bin/env node

import { createClient } from '@supabase/supabase-js';
import dotenv from 'dotenv';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
dotenv.config({ path: join(__dirname, '../.env') });

// Validate environment variables
const supabaseUrl = process.env.VITE_SUPABASE_URL || process.env.SUPABASE_URL;
const supabaseAnonKey = process.env.VITE_SUPABASE_ANON_KEY || process.env.SUPABASE_ANON_KEY;

if (!supabaseUrl || !supabaseAnonKey) {
    console.error('❌ Missing required environment variables:');
    if (!supabaseUrl) console.error('   - VITE_SUPABASE_URL or SUPABASE_URL');
    if (!supabaseAnonKey) console.error('   - VITE_SUPABASE_ANON_KEY or SUPABASE_ANON_KEY');
    console.error('\n📝 Please set these in your .env file');
    console.error('\n💡 You can find these values in your Supabase project settings:');
    console.error('   1. Go to https://app.supabase.com');
    console.error('   2. Select your project');
    console.error('   3. Go to Settings → API');
    console.error('   4. Copy the URL and anon/public key');
    process.exit(1);
}

// Create Supabase client
const supabase = createClient(supabaseUrl, supabaseAnonKey);

console.log(`
╔════════════════════════════════════════════════════════════════╗
║                                                                ║
║   🚀 Loco Platform - Supabase Setup Verification              ║
║                                                                ║
║   This script verifies your Supabase database setup           ║
║   and provides instructions for completing the setup.         ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
`);

console.log('🔍 Checking connection to Supabase...\n');

async function checkConnection() {
    try {
        // Try to query a system table
        const { data, error } = await supabase
            .from('_test_connection')
            .select('*')
            .limit(1);

        if (error && !error.message.includes('does not exist')) {
            console.error('❌ Failed to connect to Supabase:', error.message);
            console.error('\n💡 Please check your environment variables and try again.');
            return false;
        }

        console.log('✅ Successfully connected to Supabase!\n');
        console.log(`📍 Supabase URL: ${supabaseUrl}`);
        console.log(`🔑 Using anon key: ${supabaseAnonKey.substring(0, 20)}...`);
        console.log('');
        return true;
    } catch (error) {
        console.error('❌ Connection error:', error.message);
        return false;
    }
}

async function checkExistingSchema() {
    console.log('🔍 Checking for existing schema...\n');

    const tables = [
        { name: 'user_profiles', description: 'User profile information', required: true },
        { name: 'jobs', description: 'Job listings', required: true },
        { name: 'applications', description: 'Job applications', required: true },
        { name: 'saved_jobs', description: 'Saved job bookmarks', required: true },
        { name: 'job_views', description: 'Job view analytics', required: false },
        { name: 'notifications', description: 'User notifications', required: false },
        { name: 'messages', description: 'Direct messages', required: false },
        { name: 'job_templates', description: 'Job posting templates', required: false },
        { name: 'search_history', description: 'Search history tracking', required: false },
        { name: 'skills', description: 'Skills master list', required: false },
        { name: 'locations', description: 'Australian locations with coordinates', required: false }
    ];

    let existingTables = 0;
    let missingRequired = [];
    let missingOptional = [];

    for (const table of tables) {
        const { error } = await supabase
            .from(table.name)
            .select('count(*)', { count: 'exact', head: true });

        if (error && error.message.includes('does not exist')) {
            console.log(`❌ Table ${table.name.padEnd(20)} - Not found ${table.required ? '(REQUIRED)' : '(optional)'}`);
            if (table.required) {
                missingRequired.push(table);
            } else {
                missingOptional.push(table);
            }
        } else {
            console.log(`✅ Table ${table.name.padEnd(20)} - Exists`);
            existingTables++;
        }
    }

    console.log(`\n📊 Schema Status: ${existingTables}/${tables.length} tables exist\n`);

    if (missingRequired.length > 0) {
        console.log('🚨 Missing REQUIRED tables:\n');
        missingRequired.forEach(table => {
            console.log(`   - ${table.name}: ${table.description}`);
        });
        console.log('');
    }

    if (missingOptional.length > 0) {
        console.log('💡 Missing optional tables (can be added later):\n');
        missingOptional.forEach(table => {
            console.log(`   - ${table.name}: ${table.description}`);
        });
        console.log('');
    }

    return { existingTables, missingRequired, missingOptional, totalTables: tables.length };
}

async function checkSampleData() {
    console.log('📝 Checking for sample data...\n');

    try {
        const { data: jobs, count, error } = await supabase
            .from('jobs')
            .select('id, title, company, location, job_type', { count: 'exact' })
            .limit(5);

        if (error) {
            console.log('❌ Could not fetch jobs:', error.message);
            return;
        }

        if (count > 0) {
            console.log(`✅ Found ${count} job listings in the database\n`);
            
            if (jobs && jobs.length > 0) {
                console.log('📋 Sample jobs:');
                jobs.forEach(job => {
                    console.log(`   • ${job.title} at ${job.company} - ${job.location}`);
                });
                console.log('');
            }
        } else {
            console.log('⚠️  No job listings found. You may want to insert sample data.\n');
        }

        // Check for users
        const { count: userCount, error: userError } = await supabase
            .from('user_profiles')
            .select('id', { count: 'exact', head: true });

        if (!userError && userCount > 0) {
            console.log(`✅ Found ${userCount} user profile(s)\n`);
        }

    } catch (error) {
        console.error('Error checking sample data:', error);
    }
}

async function generateSetupInstructions(missingRequired) {
    if (missingRequired.length === 0) {
        console.log('🎉 All required tables are set up correctly!\n');
        return;
    }

    console.log('📚 Setup Instructions:\n');
    
    console.log('To complete your database setup, follow these steps:\n');
    
    console.log('1️⃣  Open your Supabase SQL Editor:');
    console.log(`   ${supabaseUrl.replace('/rest/v1', '')}/project/default/sql\n`);
    
    console.log('2️⃣  Copy and execute the SQL from these files in order:\n');
    
    const migrationFiles = [
        { 
            file: '20250605000001_setup_super_admin.sql',
            description: 'Creates super admin configuration'
        },
        { 
            file: '20250605000002_create_tables_simple.sql',
            description: 'Creates basic tables with RLS'
        },
        { 
            file: '20250605000003_complete_schema.sql',
            description: 'Creates comprehensive schema with all features'
        }
    ];

    migrationFiles.forEach((migration, index) => {
        console.log(`   ${String.fromCharCode(97 + index)}. ${migration.file}`);
        console.log(`      ${migration.description}`);
        console.log(`      📁 Location: ./supabase/migrations/${migration.file}\n`);
    });
    
    console.log('3️⃣  After running each migration, you should see "Success" message\n');
    
    console.log('4️⃣  Run this verification script again to confirm setup\n');

    console.log('💡 Quick Copy Commands:\n');
    console.log('   # View migration files:');
    console.log('   cat ./supabase/migrations/*.sql\n');
    console.log('   # Copy to clipboard (macOS):');
    console.log('   cat ./supabase/migrations/*.sql | pbcopy\n');
}

async function checkAuthConfiguration() {
    console.log('🔐 Checking authentication configuration...\n');

    try {
        // Try to sign up a test user (will fail if auth is not configured)
        const { data, error } = await supabase.auth.signUp({
            email: 'test@example.com',
            password: 'testpassword123'
        });

        if (error && error.message.includes('not enabled')) {
            console.log('⚠️  Email authentication is not enabled');
            console.log('   Go to Authentication → Providers in Supabase dashboard\n');
        } else if (error && error.message.includes('already registered')) {
            console.log('✅ Email authentication is enabled\n');
        } else if (data) {
            console.log('✅ Email authentication is enabled');
            // Clean up test user if created
            if (data.user) {
                await supabase.auth.admin.deleteUser(data.user.id).catch(() => {});
            }
        }
    } catch (error) {
        console.log('⚠️  Could not verify authentication setup\n');
    }
}

async function generateNextSteps(existingTables, totalTables) {
    console.log('🚀 Next Steps:\n');

    if (existingTables === totalTables) {
        console.log('Your database is fully configured! Here\'s what you can do next:\n');
        console.log('1. Start the development server:');
        console.log('   npm run dev\n');
        console.log('2. Visit the application:');
        console.log('   - Landing page: http://localhost:3080/');
        console.log('   - Dashboard: http://localhost:3080/dashboard.html');
        console.log('   - Login: http://localhost:3080/login.html\n');
        console.log('3. Create a test account and explore the features!\n');
    } else {
        console.log('After completing the database setup:\n');
        console.log('1. Run this verification script again');
        console.log('2. Start the development server with: npm run dev');
        console.log('3. Test the application functionality\n');
    }

    console.log('📖 Documentation:');
    console.log('   • README.md - Project overview and setup');
    console.log('   • SETUP.md - Detailed setup instructions');
    console.log('   • checklist.md - Development roadmap\n');
}

async function main() {
    // Check connection
    const connected = await checkConnection();
    if (!connected) {
        process.exit(1);
    }

    // Check existing schema
    const { existingTables, missingRequired, missingOptional, totalTables } = await checkExistingSchema();

    // Check sample data if tables exist
    if (existingTables > 0) {
        await checkSampleData();
    }

    // Check auth configuration
    await checkAuthConfiguration();

    // Generate setup instructions if needed
    if (missingRequired.length > 0) {
        await generateSetupInstructions(missingRequired);
    }

    // Generate next steps
    await generateNextSteps(existingTables, totalTables);

    console.log('✨ Verification complete!\n');
}

// Run the script
main().catch(console.error);