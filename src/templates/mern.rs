use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    // Root README
    write_file(&project_path.join("README.md"), ROOT_README)?;
    
    // Backend
    write_file(&project_path.join("backend/package.json"), BACKEND_PACKAGE)?;
    write_file(&project_path.join("backend/server.js"), SERVER_JS)?;
    write_file(&project_path.join("backend/.env.example"), ENV_EXAMPLE)?;
    write_file(&project_path.join("backend/models/User.js"), USER_MODEL)?;
    write_file(&project_path.join("backend/routes/api.js"), API_ROUTES)?;
    
    // Frontend
    write_file(&project_path.join("frontend/package.json"), FRONTEND_PACKAGE)?;
    write_file(&project_path.join("frontend/public/index.html"), FRONTEND_INDEX)?;
    write_file(&project_path.join("frontend/src/index.js"), FRONTEND_INDEX_JS)?;
    write_file(&project_path.join("frontend/src/App.js"), FRONTEND_APP)?;
    write_file(&project_path.join("frontend/src/App.css"), FRONTEND_CSS)?;
    
    // .gitignore
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const ROOT_README: &str = r###"# MERN Stack Application

Full-stack application built with MongoDB, Express, React, and Node.js.

## Project Structure

```
├── backend/          # Express API server
│   ├── models/       # MongoDB models
│   ├── routes/       # API routes
│   └── server.js     # Entry point
└── frontend/         # React application
    ├── public/
    └── src/
```

## Setup

### Backend

```bash
cd backend
npm install
cp .env.example .env  # Configure your environment variables
npm start
```

The backend will run on `http://localhost:5000`

### Frontend

```bash
cd frontend
npm install
npm start
```

The frontend will run on `http://localhost:3000`

## Environment Variables

Create a `.env` file in the `backend` directory:

```
MONGODB_URI=mongodb://localhost:27017/myapp
PORT=5000
JWT_SECRET=your_secret_key
```

## Features

- RESTful API with Express
- MongoDB database integration
- React frontend with hooks
- Responsive design
- User authentication ready

## License

MIT
"###;

const BACKEND_PACKAGE: &str = r###"{
  "name": "mern-backend",
  "version": "1.0.0",
  "description": "Express backend for MERN app",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "mongoose": "^8.0.0",
    "cors": "^2.8.5",
    "dotenv": "^16.3.1",
    "bcryptjs": "^2.4.3",
    "jsonwebtoken": "^9.0.2"
  },
  "devDependencies": {
    "nodemon": "^3.0.1"
  }
}
"###;

const SERVER_JS: &str = r###"const express = require('express');
const mongoose = require('mongoose');
const cors = require('cors');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 5000;

// Middleware
app.use(cors());
app.use(express.json());

// MongoDB Connection
mongoose.connect(process.env.MONGODB_URI || 'mongodb://localhost:27017/myapp', {
  useNewUrlParser: true,
  useUnifiedTopology: true,
})
.then(() => console.log('MongoDB connected'))
.catch(err => console.log('MongoDB connection error:', err));

// Routes
app.use('/api', require('./routes/api'));

app.get('/', (req, res) => {
  res.json({ message: 'Welcome to MERN API' });
});

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
"###;

const ENV_EXAMPLE: &str = r###"MONGODB_URI=mongodb://localhost:27017/myapp
PORT=5000
JWT_SECRET=your_secret_key_here
"###;

const USER_MODEL: &str = r###"const mongoose = require('mongoose');

const userSchema = new mongoose.Schema({
  name: {
    type: String,
    required: true
  },
  email: {
    type: String,
    required: true,
    unique: true
  },
  password: {
    type: String,
    required: true
  },
  createdAt: {
    type: Date,
    default: Date.now
  }
});

module.exports = mongoose.model('User', userSchema);
"###;

const API_ROUTES: &str = r###"const express = require('express');
const router = express.Router();
const User = require('../models/User');

// Get all users
router.get('/users', async (req, res) => {
  try {
    const users = await User.find().select('-password');
    res.json(users);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

// Create user
router.post('/users', async (req, res) => {
  const user = new User({
    name: req.body.name,
    email: req.body.email,
    password: req.body.password // Remember to hash in production!
  });

  try {
    const newUser = await user.save();
    res.status(201).json(newUser);
  } catch (error) {
    res.status(400).json({ message: error.message });
  }
});

// Sample data endpoint
router.get('/data', (req, res) => {
  res.json({
    message: 'API is working!',
    data: ['Item 1', 'Item 2', 'Item 3']
  });
});

module.exports = router;
"###;

const FRONTEND_PACKAGE: &str = r###"{
  "name": "mern-frontend",
  "version": "0.1.0",
  "private": true,
  "proxy": "http://localhost:5000",
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-scripts": "5.0.1",
    "axios": "^1.6.0"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}
"###;

const FRONTEND_INDEX: &str = r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#000000" />
    <meta name="description" content="MERN Stack Application" />
    <title>MERN App</title>
  </head>
  <body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
  </body>
</html>
"###;

const FRONTEND_INDEX_JS: &str = r###"import React from 'react';
import ReactDOM from 'react-dom/client';
import './App.css';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
"###;

const FRONTEND_APP: &str = r###"import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';

function App() {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      const response = await axios.get('/api/data');
      setData(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching data:', error);
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>MERN Stack Application</h1>
        <div className="content">
          {loading ? (
            <p>Loading...</p>
          ) : data ? (
            <div>
              <p>{data.message}</p>
              <ul>
                {data.data && data.data.map((item, index) => (
                  <li key={index}>{item}</li>
                ))}
              </ul>
            </div>
          ) : (
            <p>No data available. Make sure the backend is running!</p>
          )}
        </div>
        <div className="info">
          <h3>Stack:</h3>
          <ul className="stack-list">
            <li>⚡ MongoDB - Database</li>
            <li>🚀 Express - Backend Framework</li>
            <li>⚛️ React - Frontend Library</li>
            <li>💚 Node.js - Runtime</li>
          </ul>
        </div>
      </header>
    </div>
  );
}

export default App;
"###;

const FRONTEND_CSS: &str = r###".App {
  text-align: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.App-header {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: calc(10px + 2vmin);
  color: white;
  padding: 20px;
}

.App h1 {
  margin-bottom: 2rem;
  font-size: 3rem;
}

.content {
  background: rgba(255, 255, 255, 0.1);
  padding: 2rem;
  border-radius: 10px;
  margin: 2rem 0;
  min-width: 400px;
}

.content ul {
  list-style: none;
  padding: 0;
}

.content li {
  padding: 0.5rem;
  background: rgba(255, 255, 255, 0.2);
  margin: 0.5rem 0;
  border-radius: 5px;
}

.info {
  background: rgba(255, 255, 255, 0.1);
  padding: 1.5rem;
  border-radius: 10px;
  margin-top: 2rem;
}

.stack-list {
  list-style: none;
  padding: 0;
  text-align: left;
}

.stack-list li {
  padding: 0.5rem;
  margin: 0.5rem 0;
  font-size: 1.1rem;
}

@media (max-width: 768px) {
  .App h1 {
    font-size: 2rem;
  }
  
  .content {
    min-width: auto;
    width: 90%;
  }
}
"###;

const GITIGNORE: &str = r###"# Dependencies
node_modules/
/backend/node_modules
/frontend/node_modules

# Environment
.env
/backend/.env
/frontend/.env

# Production
/frontend/build

# Misc
.DS_Store
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
"###;


