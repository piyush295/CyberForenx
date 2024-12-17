document.addEventListener('DOMContentLoaded', function() {
    const loginForm = document.querySelector('form');
    
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
  });