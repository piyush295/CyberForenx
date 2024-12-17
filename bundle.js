document.addEventListener('DOMContentLoaded', function() {
    // Form validation for login
    const loginForm = document.querySelector('form[action="login"]');
    if (loginForm) {
      loginForm.addEventListener('submit', function(event) {
        event.preventDefault();
        const email = document.getElementById('email').value;
        const password = document.getElementById('password').value;
        if (!email || !password) {
          alert('Please fill in all fields');
        } else {
          // Perform login (this is just a placeholder, actual implementation will vary)
          alert('Login successful');
        }
      });
    }
  
    // Form validation for registration
    const registerForm = document.querySelector('form[action="register"]');
    if (registerForm) {
      registerForm.addEventListener('submit', function(event) {
        event.preventDefault();
        const name = document.getElementById('name').value;
        const email = document.getElementById('email').value;
        const password = document.getElementById('password').value;
        if (!name || !email || !password) {
          alert('Please fill in all fields');
        } else {
          // Perform registration (this is just a placeholder, actual implementation will vary)
          alert('Registration successful');
        }
      });
    }
  });