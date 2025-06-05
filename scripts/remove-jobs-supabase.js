#!/usr/bin/env node

/**
 * Script to remove 2/3 of job opportunities from Supabase database
 * This will keep approximately 1/3 of the jobs, removing the rest
 */

import { createClient } from '@supabase/supabase-js';
import * as dotenv from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load environment variables
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
dotenv.config({ path: join(__dirname, '..', '.env') });

// Initialize Supabase client with service role key for admin access
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

async function removeJobs() {
    try {
        console.log('🔍 Fetching all jobs from Supabase...');
        
        // Get all jobs
        const { data: jobs, error: fetchError } = await supabase
            .from('jobs')
            .select('*')
            .order('created_at', { ascending: false });

        if (fetchError) {
            console.error('❌ Error fetching jobs:', fetchError);
            return;
        }

        if (!jobs || jobs.length === 0) {
            console.log('ℹ️  No jobs found in the database');
            return;
        }

        console.log(`📊 Found ${jobs.length} total jobs`);

        // Calculate how many to keep (1/3) and how many to remove (2/3)
        const totalJobs = jobs.length;
        const jobsToKeep = Math.floor(totalJobs / 3);
        const jobsToRemove = totalJobs - jobsToKeep;

        console.log(`📌 Keeping ${jobsToKeep} jobs`);
        console.log(`🗑️  Removing ${jobsToRemove} jobs`);

        // Shuffle the array to randomly select which jobs to remove
        const shuffledJobs = [...jobs].sort(() => Math.random() - 0.5);
        
        // Get the IDs of jobs to remove (first 2/3 of shuffled array)
        const jobIdsToRemove = shuffledJobs
            .slice(0, jobsToRemove)
            .map(job => job.id);

        // Remove jobs in batches to avoid timeout
        const batchSize = 50;
        let removed = 0;

        for (let i = 0; i < jobIdsToRemove.length; i += batchSize) {
            const batch = jobIdsToRemove.slice(i, i + batchSize);
            
            const { error: deleteError } = await supabase
                .from('jobs')
                .delete()
                .in('id', batch);

            if (deleteError) {
                console.error(`❌ Error deleting batch ${i / batchSize + 1}:`, deleteError);
            } else {
                removed += batch.length;
                console.log(`✅ Deleted batch ${i / batchSize + 1}: ${batch.length} jobs (${removed}/${jobsToRemove} total)`);
            }
        }

        // Verify final count
        const { count: finalCount, error: countError } = await supabase
            .from('jobs')
            .select('*', { count: 'exact', head: true });

        if (countError) {
            console.error('❌ Error counting remaining jobs:', countError);
        } else {
            console.log(`\n✨ Success! Database now contains ${finalCount} jobs (was ${totalJobs})`);
            console.log(`📉 Removed ${removed} jobs (${((removed / totalJobs) * 100).toFixed(1)}% reduction)`);
        }

    } catch (error) {
        console.error('❌ Unexpected error:', error);
    }
}

// Run the script
console.log('🚀 Starting job removal script...\n');
removeJobs()
    .then(() => {
        console.log('\n✅ Script completed successfully');
        process.exit(0);
    })
    .catch((error) => {
        console.error('\n❌ Script failed:', error);
        process.exit(1);
    });