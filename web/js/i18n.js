// Internationalization for PaperTalk website
class I18n {
    constructor() {
        this.currentLang = this.getBrowserLanguage();
        this.translations = {
            'en': {
                // Navigation
                'nav.home': 'Home',
                'nav.demo': 'Demo',
                'nav.features': 'Features',
                'nav.download': 'Download',
                'nav.help': 'Help',
                'nav.contact': 'Contact',
                'nav.login': 'Login with WeChat',

                // Home Section
                'home.title': 'PaperTalk - Your Intelligent PDF Reading Assistant',
                'home.subtitle': 'A desktop application that helps researchers and students read and translate PDF documents with AI-powered local translation.',
                'home.download': 'Download Now',
                'home.learnMore': 'Learn More',
                'home.previewAlt': 'PaperTalk App Preview',

                // Demo Section
                'demo.title': 'See PaperTalk in Action',
                'demo.subtitle': 'Watch how PaperTalk makes PDF reading and translation effortless',
                'demo.comingSoon': 'Demo video coming soon',

                // Features Section
                'features.title': 'Powerful Features',
                'features.subtitle': 'Everything you need for efficient PDF reading and translation',
                'features.pdfReader': 'PDF Reader',
                'features.pdfReaderDesc': 'Import and read PDF files with a clean, intuitive interface',
                'features.translation': 'Text Translation',
                'features.translationDesc': 'Select text and get instant translations using local AI models',
                'features.darkMode': 'Dark Mode',
                'features.darkModeDesc': 'Comfortable reading experience with day/night mode switching',
                'features.notes': 'Notes',
                'features.notesDesc': 'Add notes to important content and organize your research',
                'features.privacy': 'Privacy First',
                'features.privacyDesc': 'All data processed locally, no cloud dependency',
                'features.fast': 'Fast Performance',
                'features.fastDesc': 'Cold start in ≤3 seconds, translation in ≤800ms',

                // Download Section
                'download.title': 'Download PaperTalk',
                'download.subtitle': 'Available for Windows, macOS, and Linux',
                'download.windows': 'Windows',
                'download.windowsDesc': 'Windows 10/11 (64-bit)',
                'download.macos': 'macOS',
                'download.macosDesc': 'macOS 10.15+ (64-bit)',
                'download.linux': 'Linux',
                'download.linuxDesc': 'Ubuntu 18.04+ (64-bit)',
                'download.download': 'Download',
                'download.requirements': 'System Requirements',
                'download.ram': 'Minimum 8GB RAM',
                'download.storage': '10GB available storage (for AI models)',
                'download.ollama': 'Ollama for AI translation',

                // Help Section
                'help.title': 'Help & FAQ',
                'help.subtitle': 'Get help with installation and usage',
                'help.q1': 'How do I install PaperTalk?',
                'help.a1': 'Download the installer for your operating system and follow the setup wizard. Make sure you have Ollama installed for translation features.',
                'help.q2': 'What AI models are supported?',
                'help.a2': 'PaperTalk works with Ollama and supports models like qwen3:8b-q4_K_M. You can download models using the Ollama CLI.',
                'help.q3': 'Is my data secure?',
                'help.a3': 'Yes! All processing happens locally on your computer. No data is sent to external servers.',
                'help.q4': 'Can I use it offline?',
                'help.a4': 'Absolutely! Once installed with the AI models, PaperTalk works completely offline.',

                // Contact Section
                'contact.title': 'Get in Touch',
                'contact.subtitle': 'We\'d love to hear your feedback and suggestions',
                'contact.name': 'Name',
                'contact.email': 'Email',
                'contact.message': 'Message',
                'contact.messagePlaceholder': 'Enter your message here...',
                'contact.submit': 'Send Message',

                // Footer
                'footer.tagline': 'Your intelligent PDF reading assistant',
                'footer.links': 'Links',
                'footer.support': 'Support',
                'footer.rights': 'All rights reserved.',

                // WeChat Modal
                'wechat.title': 'Login with WeChat',
                'wechat.scan': 'Scan QR Code with WeChat'
            },
            'zh': {
                // Navigation
                'nav.home': '首页',
                'nav.demo': '演示',
                'nav.features': '功能',
                'nav.download': '下载',
                'nav.help': '帮助',
                'nav.contact': '联系我们',
                'nav.login': '微信登录',

                // Home Section
                'home.title': 'PaperTalk - 智能PDF阅读助手',
                'home.subtitle': '一款桌面应用程序，帮助研究人员和学生使用AI驱动的本地翻译功能阅读和翻译PDF文档。',
                'home.download': '立即下载',
                'home.learnMore': '了解更多',
                'home.previewAlt': 'PaperTalk应用预览',

                // Demo Section
                'demo.title': '观看PaperTalk实际操作',
                'demo.subtitle': '了解PaperTalk如何让PDF阅读和翻译变得轻松',
                'demo.comingSoon': '演示视频即将推出',

                // Features Section
                'features.title': '强大功能',
                'features.subtitle': '高效PDF阅读和翻译所需的一切功能',
                'features.pdfReader': 'PDF阅读器',
                'features.pdfReaderDesc': '使用简洁直观的界面导入和阅读PDF文件',
                'features.translation': '文本翻译',
                'features.translationDesc': '选择文本并使用本地AI模型获得即时翻译',
                'features.darkMode': '深色模式',
                'features.darkModeDesc': '通过日间/夜间模式切换获得舒适的阅读体验',
                'features.notes': '笔记功能',
                'features.notesDesc': '为重要内容添加笔记并组织您的研究',
                'features.privacy': '隐私优先',
                'features.privacyDesc': '所有数据本地处理，无需依赖云端',
                'features.fast': '快速性能',
                'features.fastDesc': '冷启动≤3秒，翻译≤800毫秒',

                // Download Section
                'download.title': '下载PaperTalk',
                'download.subtitle': '支持Windows、macOS和Linux',
                'download.windows': 'Windows',
                'download.windowsDesc': 'Windows 10/11 (64位)',
                'download.macos': 'macOS',
                'download.macosDesc': 'macOS 10.15+ (64位)',
                'download.linux': 'Linux',
                'download.linuxDesc': 'Ubuntu 18.04+ (64位)',
                'download.download': '下载',
                'download.requirements': '系统要求',
                'download.ram': '至少8GB内存',
                'download.storage': '10GB可用存储空间（用于AI模型）',
                'download.ollama': '需要Ollama进行AI翻译',

                // Help Section
                'help.title': '帮助与常见问题',
                'help.subtitle': '获取安装和使用帮助',
                'help.q1': '如何安装PaperTalk？',
                'help.a1': '下载适用于您操作系统的安装程序，然后按照设置向导操作。确保已安装Ollama以使用翻译功能。',
                'help.q2': '支持哪些AI模型？',
                'help.a2': 'PaperTalk与Ollama配合使用，支持如qwen3:8b-q4_K_M等模型。您可以使用Ollama CLI下载模型。',
                'help.q3': '我的数据安全吗？',
                'help.a3': '是的！所有处理都在您的计算机本地进行。不会将任何数据发送到外部服务器。',
                'help.q4': '可以离线使用吗？',
                'help.a4': '完全可以！一旦安装了AI模型，PaperTalk可以完全离线工作。',

                // Contact Section
                'contact.title': '联系我们',
                'contact.subtitle': '我们很乐意听取您的反馈和建议',
                'contact.name': '姓名',
                'contact.email': '邮箱',
                'contact.message': '消息',
                'contact.messagePlaceholder': '在此输入您的消息...',
                'contact.submit': '发送消息',

                // Footer
                'footer.tagline': '您的智能PDF阅读助手',
                'footer.links': '链接',
                'footer.support': '支持',
                'footer.rights': '保留所有权利。',

                // WeChat Modal
                'wechat.title': '微信登录',
                'wechat.scan': '使用微信扫描二维码'
            }
        };
        
        this.init();
    }

    getBrowserLanguage() {
        const browserLang = navigator.language || navigator.userLanguage;
        return browserLang.startsWith('zh') ? 'zh' : 'en';
    }

    init() {
        this.updatePageLanguage();
        this.setupLanguageToggle();
    }

    updatePageLanguage() {
        // Update all elements with data-i18n attribute
        const elements = document.querySelectorAll('[data-i18n]');
        elements.forEach(element => {
            const key = element.getAttribute('data-i18n');
            if (this.translations[this.currentLang] && this.translations[this.currentLang][key]) {
                if (element.tagName === 'INPUT' || element.tagName === 'TEXTAREA') {
                    element.placeholder = this.translations[this.currentLang][key];
                } else {
                    element.textContent = this.translations[this.currentLang][key];
                }
            }
        });

        // Update language toggle button
        const toggleBtn = document.getElementById('language-toggle');
        if (toggleBtn) {
            toggleBtn.textContent = this.currentLang === 'en' ? '中文' : 'English';
        }

        // Update HTML lang attribute
        document.documentElement.lang = this.currentLang;

        // Update page title
        if (this.currentLang === 'zh') {
            document.title = 'PaperTalk - PDF阅读助手';
        } else {
            document.title = 'PaperTalk - PDF Reading Assistant';
        }
    }

    setupLanguageToggle() {
        const toggleBtn = document.getElementById('language-toggle');
        if (toggleBtn) {
            toggleBtn.addEventListener('click', () => {
                this.currentLang = this.currentLang === 'en' ? 'zh' : 'en';
                this.updatePageLanguage();
                
                // Save language preference
                localStorage.setItem('preferred-language', this.currentLang);
            });
        }

        // Load saved language preference
        const savedLang = localStorage.getItem('preferred-language');
        if (savedLang && (savedLang === 'en' || savedLang === 'zh')) {
            this.currentLang = savedLang;
            this.updatePageLanguage();
        }
    }

    getText(key) {
        return this.translations[this.currentLang]?.[key] || key;
    }
}

// Initialize i18n when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.i18n = new I18n();
});
