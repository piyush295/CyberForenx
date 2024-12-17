document.addEventListener('DOMContentLoaded', function() {
    const contactForm = document.querySelector('form');
    
    if (contactForm) {
      contactForm.addEventListener('submit', function(event) {
        event.preventDefault();
        
        const name = document.getElementById('name').value;
        const email = document.getElementById('email').value;
        const message = document.getElementById('message').value;
        
        if (!name || !email || !message) {
          alert('Please fill in all fields');
        } else {
          // Perform form submission (this is just a placeholder, actual implementation will vary)
          alert('Message sent successfully');
        }
      });
    }
  });