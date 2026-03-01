// API Configuration
const API_BASE_URL = window.location.origin;

// State management
let authToken = localStorage.getItem('authToken');
let currentUser = null;

// DOM Elements
const loginPage = document.getElementById('login-page');
const registerPage = document.getElementById('register-page');
const dashboardPage = document.getElementById('dashboard-page');
const loginForm = document.getElementById('login-form');
const registerForm = document.getElementById('register-form');
const showRegisterBtn = document.getElementById('show-register');
const showLoginBtn = document.getElementById('show-login');
const logoutBtn = document.getElementById('logout-btn');
const refreshBtn = document.getElementById('refresh-btn');
const createKeyBtn = document.getElementById('create-key-btn');
const createKeyModal = document.getElementById('create-key-modal');
const showKeyModal = document.getElementById('show-key-modal');
const createKeyForm = document.getElementById('create-key-form');

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    if (authToken) {
        loadDashboard();
    } else {
        showPage('login');
    }

    setupEventListeners();
});

// Event Listeners
function setupEventListeners() {
    loginForm.addEventListener('submit', handleLogin);
    registerForm.addEventListener('submit', handleRegister);
    showRegisterBtn.addEventListener('click', (e) => {
        e.preventDefault();
        showPage('register');
    });
    showLoginBtn.addEventListener('click', (e) => {
        e.preventDefault();
        showPage('login');
    });
    logoutBtn.addEventListener('click', handleLogout);
    refreshBtn.addEventListener('click', loadDashboard);
    createKeyBtn.addEventListener('click', () => showModal('create-key'));
    createKeyForm.addEventListener('submit', handleCreateApiKey);

    // Modal close buttons
    document.querySelectorAll('.modal-close, .modal-cancel').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.preventDefault();
            closeAllModals();
        });
    });

    // Copy key button
    document.getElementById('copy-key-btn').addEventListener('click', () => {
        const keyText = document.getElementById('new-api-key').textContent;
        navigator.clipboard.writeText(keyText).then(() => {
            const btn = document.getElementById('copy-key-btn');
            const originalText = btn.textContent;
            btn.textContent = '✓ Copied!';
            setTimeout(() => {
                btn.textContent = originalText;
            }, 2000);
        });
    });

    // Close modal on background click
    [createKeyModal, showKeyModal].forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                closeAllModals();
            }
        });
    });
}

// Page navigation
function showPage(page) {
    loginPage.classList.add('hidden');
    registerPage.classList.add('hidden');
    dashboardPage.classList.add('hidden');

    switch (page) {
        case 'login':
            loginPage.classList.remove('hidden');
            break;
        case 'register':
            registerPage.classList.remove('hidden');
            break;
        case 'dashboard':
            dashboardPage.classList.remove('hidden');
            break;
    }
}

// Authentication handlers
async function handleLogin(e) {
    e.preventDefault();
    const email = document.getElementById('email').value;
    const password = document.getElementById('password').value;
    const errorElement = document.getElementById('login-error');

    errorElement.classList.remove('show');

    try {
        const response = await fetch(`${API_BASE_URL}/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Login failed');
        }

        authToken = data.access_token;
        localStorage.setItem('authToken', authToken);
        loadDashboard();
    } catch (error) {
        errorElement.textContent = error.message;
        errorElement.classList.add('show');
    }
}

async function handleRegister(e) {
    e.preventDefault();
    const email = document.getElementById('register-email').value;
    const password = document.getElementById('register-password').value;
    const errorElement = document.getElementById('register-error');
    const successElement = document.getElementById('register-success');

    errorElement.classList.remove('show');
    successElement.classList.remove('show');

    try {
        const response = await fetch(`${API_BASE_URL}/auth/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Registration failed');
        }

        successElement.textContent = 'Registration successful! Please login.';
        successElement.classList.add('show');
        registerForm.reset();

        // Switch to login page after 2 seconds
        setTimeout(() => {
            showPage('login');
        }, 2000);
    } catch (error) {
        errorElement.textContent = error.message;
        errorElement.classList.add('show');
    }
}

function handleLogout() {
    authToken = null;
    currentUser = null;
    localStorage.removeItem('authToken');
    showPage('login');
}

// Dashboard
async function loadDashboard() {
    try {
        await loadUserInfo();
        await loadApiKeys();
        showPage('dashboard');
    } catch (error) {
        console.error('Failed to load dashboard:', error);
        handleLogout();
    }
}

async function loadUserInfo() {
    try {
        const response = await fetch(`${API_BASE_URL}/auth/me`, {
            headers: {
                'Authorization': `Bearer ${authToken}`,
            },
        });

        if (!response.ok) {
            throw new Error('Failed to load user info');
        }

        currentUser = await response.json();

        // Update UI
        document.getElementById('user-email').textContent = currentUser.email;
        document.getElementById('user-id').textContent = currentUser.id;
        document.getElementById('user-email-detail').textContent = currentUser.email;
        document.getElementById('user-created').textContent = new Date(currentUser.created_at).toLocaleString();
    } catch (error) {
        console.error('Failed to load user info:', error);
        throw error;
    }
}

async function loadApiKeys() {
    const apiKeysList = document.getElementById('api-keys-list');
    apiKeysList.innerHTML = '<div class="loading">Loading API keys...</div>';

    try {
        const response = await fetch(`${API_BASE_URL}/auth/api-keys`, {
            headers: {
                'Authorization': `Bearer ${authToken}`,
            },
        });

        if (!response.ok) {
            throw new Error('Failed to load API keys');
        }

        const apiKeys = await response.json();

        if (apiKeys.length === 0) {
            apiKeysList.innerHTML = `
                <div class="empty-state">
                    <h4>No API Keys</h4>
                    <p>Create your first API key to get started.</p>
                </div>
            `;
            return;
        }

        apiKeysList.innerHTML = apiKeys.map(key => `
            <div class="api-key-item ${key.is_active ? '' : 'inactive'}">
                <div class="api-key-info">
                    <div class="api-key-name">${escapeHtml(key.name)}</div>
                    <div class="api-key-details">
                        <span>ID: ${key.id}</span>
                        <span>Created: ${new Date(key.created_at).toLocaleString()}</span>
                        ${key.last_used_at ? `<span>Last Used: ${new Date(key.last_used_at).toLocaleString()}</span>` : '<span>Never Used</span>'}
                        ${key.expires_at ? `<span>Expires: ${new Date(key.expires_at).toLocaleString()}</span>` : '<span>Never Expires</span>'}
                        <span class="status-badge ${key.is_active ? 'status-active' : 'status-inactive'}">
                            ${key.is_active ? 'Active' : 'Revoked'}
                        </span>
                    </div>
                </div>
                <div class="api-key-actions">
                    ${key.is_active ? `<button class="btn btn-danger" onclick="revokeApiKey('${key.id}')">Revoke</button>` : ''}
                </div>
            </div>
        `).join('');
    } catch (error) {
        console.error('Failed to load API keys:', error);
        apiKeysList.innerHTML = '<div class="error-message show">Failed to load API keys. Please try again.</div>';
    }
}

// API Key Management
async function handleCreateApiKey(e) {
    e.preventDefault();
    const name = document.getElementById('key-name').value;
    const expiresAt = document.getElementById('key-expires').value;
    const errorElement = document.getElementById('create-key-error');

    errorElement.classList.remove('show');

    try {
        const payload = { name };
        if (expiresAt) {
            payload.expires_at = new Date(expiresAt).toISOString();
        }

        const response = await fetch(`${API_BASE_URL}/auth/api-keys`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${authToken}`,
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Failed to create API key');
        }

        // Show the new key to the user
        document.getElementById('new-api-key').textContent = data.key;
        closeModal('create-key');
        showModal('show-key');
        createKeyForm.reset();

        // Reload API keys list
        await loadApiKeys();
    } catch (error) {
        errorElement.textContent = error.message;
        errorElement.classList.add('show');
    }
}

async function revokeApiKey(keyId) {
    if (!confirm('Are you sure you want to revoke this API key? This action cannot be undone.')) {
        return;
    }

    try {
        const response = await fetch(`${API_BASE_URL}/auth/api-keys/${keyId}`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${authToken}`,
            },
        });

        if (!response.ok) {
            const data = await response.json();
            throw new Error(data.error || 'Failed to revoke API key');
        }

        // Reload API keys list
        await loadApiKeys();
    } catch (error) {
        alert('Failed to revoke API key: ' + error.message);
    }
}

// Modal management
function showModal(modalId) {
    const modal = document.getElementById(`${modalId}-modal`);
    if (modal) {
        modal.classList.remove('hidden');
    }
}

function closeModal(modalId) {
    const modal = document.getElementById(`${modalId}-modal`);
    if (modal) {
        modal.classList.add('hidden');
    }
}

function closeAllModals() {
    document.querySelectorAll('.modal').forEach(modal => {
        modal.classList.add('hidden');
    });
}

// Utility functions
function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, m => map[m]);
}
