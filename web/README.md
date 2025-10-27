# PaperTalk Website

This is the official website for PaperTalk - an intelligent PDF reading assistant desktop application.

## Features

- **Bilingual Support**: Automatically switches between English and Chinese based on browser language
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **WeChat Login**: Integrated WeChat authentication system
- **Modern UI**: Clean, professional design with smooth animations
- **Contact Form**: User feedback and suggestion collection

## Website Structure

### Pages/Sections

1. **Home** - Introduction and main call-to-action
2. **Demo** - Video demonstration (placeholder for now)
3. **Features** - Detailed feature showcase
4. **Download** - Platform-specific download options
5. **Help** - FAQ and installation guidance
6. **Contact** - Contact form for user feedback

### File Structure

```
web/
├── index.html          # Main HTML file
├── css/
│   └── style.css       # All styles and responsive design
├── js/
│   ├── i18n.js         # Internationalization (English/Chinese)
│   └── script.js       # Main JavaScript functionality
├── images/             # Image assets (placeholders)
└── README.md           # This file
```

## Technical Implementation

### Internationalization (i18n)

The website automatically detects browser language and displays content in:
- **English** (default for non-Chinese browsers)
- **Chinese** (for browsers with Chinese language preference)

Language can be manually toggled using the language button in the navigation.

### WeChat Login

- Modal-based WeChat QR code authentication
- Simulated login process for demonstration
- Responsive design for mobile devices

### Responsive Design

- Mobile-first approach
- Breakpoints for tablets and mobile devices
- Hamburger menu for mobile navigation

### Interactive Features

- Smooth scrolling navigation
- Form validation and submission
- Download button functionality
- Scroll-triggered animations
- Notification system

## Setup and Deployment

### Local Development

1. Navigate to the `web` directory
2. Serve the files using a local web server:
   ```bash
   # Using Python 3
   python -m http.server 8000
   
   # Using Node.js (if you have http-server installed)
   npx http-server
   
   # Using PHP
   php -S localhost:8000
   ```

3. Open `http://localhost:8000` in your browser

### Production Deployment

The website consists of static files and can be deployed to any web hosting service:

- **Netlify**: Drag and drop the `web` folder
- **Vercel**: Connect your Git repository
- **GitHub Pages**: Push to a GitHub repository
- **Traditional Web Hosting**: Upload files via FTP/SFTP

## Customization

### Adding New Languages

1. Edit `js/i18n.js`
2. Add a new language object to the `translations` object
3. Update the language detection logic if needed

### Styling

- CSS variables in `:root` for easy theming
- Modular CSS structure
- Responsive breakpoints at 768px and 480px

### Content Updates

- Text content is managed through the i18n system
- Images should be placed in the `images` directory
- Feature cards can be added/removed in the HTML

## Browser Support

- Chrome 60+
- Firefox 55+
- Safari 12+
- Edge 79+
- Mobile browsers (iOS Safari, Chrome Mobile)

## Performance Features

- Lazy loading for images
- Optimized animations
- Efficient JavaScript execution
- Minimal external dependencies

## License

This website is part of the PaperTalk project. See the main project README for license information.
