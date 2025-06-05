#!/usr/bin/env node

/**
 * Script to check available tables in Supabase database
 */

import { createClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load environment variables
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
dotenv.config({ path: join(__dirname, '..', '.env') });

// Initialize Supabase client
const supabaseUrl = process.env.VITE_SUPABASE_URL;
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;

if (!supabaseUrl || !supabaseServiceKey) {
    console.error('❌ Missing Supabase configuration in .env file');
    process.exit(1);
}

const supabase = createClient(supabaseUrl, supabaseServiceKey, {
    auth: {
        autoRefreshToken: false,
        persistSession: false
    }
});

async function checkTables() {
    console.log('🔍 Checking Supabase database schema...\n');
    
    // Try common table names
    const possibleTableNames = [
        'jobs',
        'job',
        'job_opportunities',
        'job_listings',
        'opportunities',
        'positions',
        'vacancy',
        'vacancies',
        'pharmacy_jobs',
        'Job',
        'Jobs'
    ];

    console.log('📋 Testing possible table names:');
    
    for (const tableName of possibleTableNames) {
        try {
            const { data, error, count } = await supabase
                .from(tableName)
                .select('*', { count: 'exact', head: true });

            if (!error) {
                console.log(`✅ Found table: "${tableName}" (${count} records)`);
                
                // Get sample record to see structure
                const { data: sample } = await supabase
                    .from(tableName)
                    .select('*')
                    .limit(1);
                
                if (sample && sample.length > 0) {
                    console.log(`   Sample columns: ${Object.keys(sample[0]).join(', ')}`);
                }
            }
        } catch (e) {
            // Table doesn't exist, continue
        }
    }

    // Also try to get schema information
    console.log('\n📊 Attempting to fetch schema information...');
    
    try {
        // Try to query the information schema (this might not work depending on permissions)
        const { data: tables, error } = await supabase.rpc('get_tables_list', {});
        
        if (!error && tables) {
            console.log('Tables found:', tables);
        }
    } catch (e) {
        console.log('ℹ️  Cannot access schema information directly');
    }
}

// Run the script
checkTables()
    .then(() => {
        console.log('\n✅ Check completed');
        process.exit(0);
    })
    .catch((error) => {
        console.error('\n❌ Script failed:', error);
        process.exit(1);
    });