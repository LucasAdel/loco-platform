#!/usr/bin/env node

/**
 * Script to verify job locations are correctly set
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

const supabase = createClient(supabaseUrl, supabaseServiceKey, {
    auth: {
        autoRefreshToken: false,
        persistSession: false
    }
});

async function verifyJobLocations() {
    console.log('🔍 Verifying job locations...\n');
    
    const { data: jobs, error } = await supabase
        .from('jobs')
        .select('id, title, suburb, postcode, latitude, longitude')
        .order('suburb');

    if (error) {
        console.error('Error fetching jobs:', error);
        return;
    }

    // Check for issues
    const issues = {
        missingCoords: [],
        invalidCoords: [],
        missingSuburb: [],
        allInOnePlace: new Map()
    };

    // Count locations
    const locationCounts = new Map();
    
    jobs.forEach(job => {
        // Check missing coordinates
        if (!job.latitude || !job.longitude) {
            issues.missingCoords.push(job);
        }
        
        // Check invalid coordinates (not in Adelaide area)
        else if (job.latitude < -35.35 || job.latitude > -34.60 || 
                 job.longitude < 138.45 || job.longitude > 138.85) {
            issues.invalidCoords.push(job);
        }
        
        // Check missing suburb
        if (!job.suburb || job.suburb === 'null' || job.suburb === 'Unknown') {
            issues.missingSuburb.push(job);
        }
        
        // Count jobs at each location
        const coordKey = `${job.latitude?.toFixed(4)},${job.longitude?.toFixed(4)}`;
        locationCounts.set(coordKey, (locationCounts.get(coordKey) || 0) + 1);
    });
    
    // Find locations with too many jobs
    locationCounts.forEach((count, coords) => {
        if (count > 10) {
            issues.allInOnePlace.set(coords, count);
        }
    });

    // Report results
    console.log(`📊 Total jobs checked: ${jobs.length}`);
    console.log(`✅ Jobs with valid coordinates: ${jobs.length - issues.missingCoords.length - issues.invalidCoords.length}`);
    
    if (issues.missingCoords.length > 0) {
        console.log(`\n❌ Jobs missing coordinates: ${issues.missingCoords.length}`);
        issues.missingCoords.slice(0, 5).forEach(job => {
            console.log(`   - "${job.title}" in ${job.suburb || 'Unknown'}`);
        });
    }
    
    if (issues.invalidCoords.length > 0) {
        console.log(`\n❌ Jobs with invalid coordinates: ${issues.invalidCoords.length}`);
        issues.invalidCoords.slice(0, 5).forEach(job => {
            console.log(`   - "${job.title}" at (${job.latitude}, ${job.longitude})`);
        });
    }
    
    if (issues.missingSuburb.length > 0) {
        console.log(`\n⚠️  Jobs without suburb: ${issues.missingSuburb.length}`);
    }
    
    if (issues.allInOnePlace.size > 0) {
        console.log(`\n⚠️  Locations with too many jobs:`);
        issues.allInOnePlace.forEach((count, coords) => {
            console.log(`   - ${count} jobs at ${coords}`);
        });
    }
    
    // Show suburb distribution summary
    const suburbCounts = new Map();
    jobs.forEach(job => {
        if (job.suburb) {
            suburbCounts.set(job.suburb, (suburbCounts.get(job.suburb) || 0) + 1);
        }
    });
    
    console.log(`\n📍 Jobs distributed across ${suburbCounts.size} suburbs`);
    console.log('Top 10 suburbs:');
    const sortedSuburbs = Array.from(suburbCounts.entries())
        .sort((a, b) => b[1] - a[1])
        .slice(0, 10);
    
    sortedSuburbs.forEach(([suburb, count]) => {
        console.log(`   ${suburb}: ${count} jobs`);
    });
    
    // Check coordinate diversity
    const uniqueCoords = new Set(jobs.map(j => `${j.latitude?.toFixed(4)},${j.longitude?.toFixed(4)}`));
    console.log(`\n🗺️  Unique coordinate locations: ${uniqueCoords.size}`);
    
    if (uniqueCoords.size < jobs.length * 0.3) {
        console.log('⚠️  Warning: Jobs may be too clustered. Consider more distribution.');
    } else {
        console.log('✅ Good distribution of job locations!');
    }
}

verifyJobLocations()
    .then(() => {
        console.log('\n✅ Verification complete');
        process.exit(0);
    })
    .catch((error) => {
        console.error('\n❌ Verification failed:', error);
        process.exit(1);
    });