#!/usr/bin/env python3
"""
Loco Platform - Demo Backend API Server
Provides mock REST API endpoints for frontend development and testing
"""

import json
import time
import uuid
from datetime import datetime, timedelta
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import threading

class LocoAPIHandler(BaseHTTPRequestHandler):
    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.end_headers()

    def do_GET(self):
        """Handle GET requests"""
        path = urlparse(self.path).path
        query = parse_qs(urlparse(self.path).query)
        
        try:
            if path == '/api/health':
                self.send_json_response({
                    "status": "healthy",
                    "service": "loco-platform-demo",
                    "version": "0.1.0",
                    "timestamp": datetime.now().isoformat()
                })
                
            elif path == '/api/jobs':
                self.handle_jobs_list(query)
                
            elif path.startswith('/api/jobs/'):
                job_id = path.split('/')[-1]
                self.handle_job_detail(job_id)
                
            elif path == '/api/auth/profile':
                self.handle_user_profile()
                
            elif path == '/api/notifications':
                self.handle_notifications()
                
            elif path == '/api/availability':
                self.handle_availability()
                
            elif path == '/api/admin/stats':
                self.handle_admin_stats()
                
            else:
                self.send_json_response({"error": "Not found"}, 404)
                
        except Exception as e:
            self.send_json_response({"error": str(e)}, 500)

    def handle_jobs_list(self, query):
        """Return paginated jobs list with filtering"""
        # Parse query parameters
        search = query.get('search', [''])[0]
        location = query.get('location', [''])[0]
        job_type = query.get('type', [''])[0]
        page = int(query.get('page', ['1'])[0])
        per_page = int(query.get('per_page', ['10'])[0])
        
        # Mock jobs data
        base_jobs = [
            {
                "id": str(uuid.uuid4()),
                "title": "Senior Pharmacist",
                "company": "Pharmacy Plus Adelaide",
                "description": "Leading pharmacy role in Adelaide CBD with excellent benefits",
                "location": "Adelaide CBD, SA",
                "postcode": "5000",
                "salary_min": 75000,
                "salary_max": 95000,
                "salary_display": "$75-95k + super",
                "job_type": "permanent",
                "experience_level": "senior",
                "urgent": False,
                "remote_ok": False,
                "created_at": (datetime.now() - timedelta(hours=2)).isoformat(),
                "expires_at": (datetime.now() + timedelta(days=30)).isoformat(),
                "applications_count": 12,
                "views_count": 156,
                "featured": True
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Locum Pharmacist - Urgent",
                "company": "MediCare Pharmacy Network",
                "description": "Immediate start required for busy community pharmacy",
                "location": "Brighton, SA",
                "postcode": "5048",
                "salary_min": 40,
                "salary_max": 55,
                "salary_display": "$40-55/hr",
                "job_type": "locum",
                "experience_level": "mid",
                "urgent": True,
                "remote_ok": False,
                "created_at": (datetime.now() - timedelta(minutes=30)).isoformat(),
                "expires_at": (datetime.now() + timedelta(days=7)).isoformat(),
                "applications_count": 3,
                "views_count": 89,
                "featured": False
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Graduate Pharmacist Program",
                "company": "Terry White Chemmart",
                "description": "12-month graduate program with mentorship and training",
                "location": "Multiple Locations, SA",
                "postcode": "5000",
                "salary_min": 55000,
                "salary_max": 65000,
                "salary_display": "$55-65k + benefits",
                "job_type": "graduate",
                "experience_level": "entry",
                "urgent": False,
                "remote_ok": False,
                "created_at": (datetime.now() - timedelta(hours=6)).isoformat(),
                "expires_at": (datetime.now() + timedelta(days=21)).isoformat(),
                "applications_count": 28,
                "views_count": 342,
                "featured": True
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Clinical Pharmacist - Hospital",
                "company": "Royal Adelaide Hospital",
                "description": "Clinical pharmacy role in major teaching hospital",
                "location": "Adelaide, SA",
                "postcode": "5000",
                "salary_min": 80000,
                "salary_max": 105000,
                "salary_display": "$80-105k + super",
                "job_type": "permanent",
                "experience_level": "senior",
                "urgent": False,
                "remote_ok": False,
                "created_at": (datetime.now() - timedelta(hours=12)).isoformat(),
                "expires_at": (datetime.now() + timedelta(days=45)).isoformat(),
                "applications_count": 8,
                "views_count": 203,
                "featured": False
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Pharmacy Assistant",
                "company": "Chemist Warehouse",
                "description": "Part-time pharmacy assistant role with flexible hours",
                "location": "Marion, SA",
                "postcode": "5043",
                "salary_min": 25,
                "salary_max": 32,
                "salary_display": "$25-32/hr",
                "job_type": "part-time",
                "experience_level": "entry",
                "urgent": False,
                "remote_ok": False,
                "created_at": (datetime.now() - timedelta(days=1)).isoformat(),
                "expires_at": (datetime.now() + timedelta(days=14)).isoformat(),
                "applications_count": 15,
                "views_count": 127,
                "featured": False
            }
        ]
        
        # Apply filters
        filtered_jobs = base_jobs
        if search:
            filtered_jobs = [j for j in filtered_jobs if search.lower() in j['title'].lower() or search.lower() in j['description'].lower()]
        if location:
            filtered_jobs = [j for j in filtered_jobs if location.lower() in j['location'].lower()]
        if job_type:
            filtered_jobs = [j for j in filtered_jobs if j['job_type'] == job_type]
        
        # Pagination
        total = len(filtered_jobs)
        start = (page - 1) * per_page
        end = start + per_page
        jobs_page = filtered_jobs[start:end]
        
        response = {
            "jobs": jobs_page,
            "pagination": {
                "page": page,
                "per_page": per_page,
                "total": total,
                "pages": (total + per_page - 1) // per_page
            },
            "filters": {
                "search": search,
                "location": location,
                "job_type": job_type
            }
        }
        
        self.send_json_response(response)

    def handle_job_detail(self, job_id):
        """Return detailed job information"""
        job = {
            "id": job_id,
            "title": "Senior Pharmacist",
            "company": "Pharmacy Plus Adelaide",
            "company_logo": "https://via.placeholder.com/100x100",
            "description": """
            <p>We are seeking an experienced Senior Pharmacist to join our team at our Adelaide CBD location.</p>
            
            <h3>Key Responsibilities:</h3>
            <ul>
                <li>Provide clinical pharmacy services</li>
                <li>Supervise junior staff and students</li>
                <li>Maintain accurate dispensing records</li>
                <li>Participate in medication reviews</li>
            </ul>
            
            <h3>Requirements:</h3>
            <ul>
                <li>AHPRA registration as a Pharmacist</li>
                <li>5+ years community pharmacy experience</li>
                <li>Strong communication skills</li>
                <li>Leadership experience preferred</li>
            </ul>
            """,
            "location": "Adelaide CBD, SA",
            "postcode": "5000",
            "address": "123 King William Street, Adelaide SA 5000",
            "coordinates": {"lat": -34.9285, "lng": 138.6007},
            "salary_min": 75000,
            "salary_max": 95000,
            "salary_display": "$75-95k + super",
            "job_type": "permanent",
            "experience_level": "senior",
            "hours_per_week": 38,
            "benefits": ["Health insurance", "Professional development", "Parking", "Flexible hours"],
            "urgent": False,
            "remote_ok": False,
            "created_at": (datetime.now() - timedelta(hours=2)).isoformat(),
            "expires_at": (datetime.now() + timedelta(days=30)).isoformat(),
            "applications_count": 12,
            "views_count": 156,
            "featured": True,
            "contact": {
                "name": "Sarah Johnson",
                "email": "careers@pharmacyplus.com.au",
                "phone": "(08) 8123 4567"
            }
        }
        
        self.send_json_response(job)

    def handle_user_profile(self):
        """Return user profile information"""
        profile = {
            "id": str(uuid.uuid4()),
            "email": "demo@locoplatform.com.au",
            "first_name": "Demo",
            "last_name": "User",
            "phone": "0412 345 678",
            "registration_number": "PHA1234567",
            "qualification": "Bachelor of Pharmacy",
            "experience_years": 5,
            "location": "Adelaide, SA",
            "availability": "full-time",
            "specializations": ["Community Pharmacy", "Clinical Pharmacy"],
            "profile_complete": 85,
            "created_at": (datetime.now() - timedelta(days=90)).isoformat(),
            "last_login": (datetime.now() - timedelta(hours=1)).isoformat()
        }
        
        self.send_json_response(profile)

    def handle_notifications(self):
        """Return user notifications"""
        notifications = [
            {
                "id": str(uuid.uuid4()),
                "title": "New Job Match",
                "message": "3 new jobs match your preferences",
                "type": "job_match",
                "read": False,
                "created_at": (datetime.now() - timedelta(minutes=15)).isoformat()
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Application Update",
                "message": "Your application for Senior Pharmacist has been viewed",
                "type": "application",
                "read": False,
                "created_at": (datetime.now() - timedelta(hours=2)).isoformat()
            },
            {
                "id": str(uuid.uuid4()),
                "title": "Profile Update",
                "message": "Please complete your profile to increase visibility",
                "type": "profile",
                "read": True,
                "created_at": (datetime.now() - timedelta(days=1)).isoformat()
            }
        ]
        
        self.send_json_response({"notifications": notifications})

    def handle_availability(self):
        """Return availability calendar data"""
        availability = {
            "calendar": [
                {
                    "date": (datetime.now() + timedelta(days=i)).strftime("%Y-%m-%d"),
                    "available": i % 3 != 0,  # Make some days unavailable
                    "shifts": [
                        {"start": "09:00", "end": "17:00", "type": "full-day"},
                        {"start": "18:00", "end": "22:00", "type": "evening"}
                    ] if i % 3 != 0 else []
                } for i in range(30)
            ],
            "preferences": {
                "min_hours_per_week": 20,
                "max_hours_per_week": 40,
                "preferred_days": ["monday", "tuesday", "wednesday", "thursday", "friday"],
                "travel_distance": 25
            }
        }
        
        self.send_json_response(availability)

    def handle_admin_stats(self):
        """Return admin dashboard statistics"""
        stats = {
            "jobs": {
                "total": 1247,
                "active": 89,
                "filled_this_month": 156,
                "applications_this_month": 892
            },
            "users": {
                "total": 3456,
                "active_this_month": 1234,
                "new_this_month": 89,
                "employers": 234
            },
            "platform": {
                "uptime": "99.9%",
                "response_time": "45ms",
                "last_deployment": (datetime.now() - timedelta(days=3)).isoformat()
            }
        }
        
        self.send_json_response(stats)

    def send_json_response(self, data, status_code=200):
        """Send JSON response with proper headers"""
        json_data = json.dumps(data, indent=2).encode('utf-8')
        
        self.send_response(status_code)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(json_data)))
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.end_headers()
        self.wfile.write(json_data)

    def log_message(self, format, *args):
        """Override to provide better logging"""
        print(f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] {self.client_address[0]} - {format % args}")

def run_server(port=3000):
    """Run the demo API server"""
    server_address = ('', port)
    httpd = HTTPServer(server_address, LocoAPIHandler)
    
    print(f"🚀 Loco Platform Demo API Server")
    print(f"📡 Running on http://localhost:{port}")
    print(f"🏥 Australian Pharmacy Jobs API")
    print(f"📊 Mock endpoints available:")
    print(f"   • GET  /api/health           - Health check")
    print(f"   • GET  /api/jobs             - Jobs listing with filters")
    print(f"   • GET  /api/jobs/:id         - Job details")
    print(f"   • GET  /api/auth/profile     - User profile")
    print(f"   • GET  /api/notifications    - User notifications")
    print(f"   • GET  /api/availability     - Availability calendar")
    print(f"   • GET  /api/admin/stats      - Admin statistics")
    print(f"💻 CORS enabled for development")
    print(f"🔄 Auto-generates realistic demo data")
    print("")
    
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Server stopped gracefully")
        httpd.server_close()

if __name__ == "__main__":
    run_server()