// Pasted_Text_1754161025956.txt
import React, { useState, useEffect } from 'react';
import clsx from 'clsx';
import styles from './index.module.css';
import { FaBolt, FaBullseye, FaWrench, FaChartBar, FaSyncAlt, FaPuzzlePiece } from 'react-icons/fa';

export default function Home() {
  // Hardcode colorMode to 'dark' as light mode is no longer supported
  const colorMode = 'dark'; // Removed state and setter
  const [currentCommand, setCurrentCommand] = useState(0);
  const [typedText, setTypedText] = useState('');
  const [isTyping, setIsTyping] = useState(true);

  // Removed the useEffect hook that initialized theme from localStorage/system preference
  // Removed the toggleColorMode function

  const commands = [
    // Add commands
    'gh-templates issue add bug',
    'gh-templates license add mit',
    'gh-templates gitignore add node',
    'gh-templates pr add default',
    // List commands
    'gh-templates issue list',
    'gh-templates license list',
    'gh-templates license list --popular',
    'gh-templates license list --non-software',
    'gh-templates license list --fsf-libre',
    'gh-templates license list --osi-approved',
    'gh-templates license list --search apa*',
    'gh-templates gitignore list',
    'gh-templates gitignore list --popular',
    'gh-templates gitignore list --global',
    'gh-templates gitignore list --community',
    'gh-templates pr list',
    // Preview commands
    'gh-templates issue preview bug',
    'gh-templates license preview mit',
    'gh-templates gitignore preview node',
    'gh-templates pr preview default',
  ];

  const features = [
    {
      icon: <FaBolt />,
      title: 'Lightning Fast',
      description: 'Generate professional GitHub templates in seconds, not minutes. Built with Rust for maximum performance.',
      highlight: 'Rust-powered'
    },
    {
      icon: <FaBullseye />,
      title: 'Smart Templates',
      description: 'AI-curated templates that follow GitHub best practices and community standards.',
      highlight: 'Best Practices'
    },
    {
      icon: <FaWrench />,
      title: 'Fully Customizable',
      description: 'Tailor every template to your project needs with powerful customization options.',
      highlight: 'Your Way'
    },
    {
      icon: <FaChartBar />,
      title: 'Preview Mode',
      description: 'See exactly what your templates will look like before committing to your repository. With syntax highlighting and formatting.',
      highlight: 'Zero Surprises'
    },
    {
      icon: <FaSyncAlt />,
      title: 'Version Sync',
      description: 'Keep your templates up-to-date with the latest GitHub features and community trends.',
      highlight: 'Always Current'
    },
    {
      icon: <FaPuzzlePiece />,
      title: 'Custom Sources & Groups',
      description: "Add, organize, and apply templates from custom sources and groups for ultimate flexibility.",
      highlight: 'Customizable Sources & Groups (coming soon)'
    }
  ];

  // Typewriter effect
  useEffect(() => {
    const currentCmd = commands[currentCommand];
    let typingTimeout;
    let pauseTimeout;
    let resetTimeout;

    if (isTyping && typedText.length < currentCmd.length) {
      typingTimeout = setTimeout(() => {
        setTypedText(currentCmd.slice(0, typedText.length + 1));
      }, 100);
    } else if (isTyping && typedText.length === currentCmd.length) {
      // Command is fully typed, pause before clearing
      pauseTimeout = setTimeout(() => {
        setIsTyping(false);
      }, 1000);
    } else if (!isTyping && typedText.length > 0) {
      // Clear text and move to next command
      resetTimeout = setTimeout(() => {
        setTypedText('');
        setCurrentCommand((prev) => (prev + 1) % commands.length);
        setIsTyping(true);
      }, 2000);
    }

    // Cleanup function
    return () => {
      if (typingTimeout) clearTimeout(typingTimeout);
      if (pauseTimeout) clearTimeout(pauseTimeout);
      if (resetTimeout) clearTimeout(resetTimeout);
    };
  }, [typedText, isTyping, currentCommand, commands]);

  // Simplified check for dark mode (always true)
  const isDark = true; // colorMode === 'dark';

  return (
    <div className={clsx(styles.container, isDark && styles.dark)}>
      {/* Theme Toggle - Kept for layout but made non-functional */}
      <div className={styles.themeToggle}>
      </div>

      {/* Hero Section */}
      <div className={styles.heroSection}>
        {/* Background Pattern */}
        <div className={clsx(styles.backgroundPattern, isDark && styles.backgroundPatternDark)} />
        <div className={styles.heroContent}>
          <div className={styles.heroText}>
            {/* Main Heading */}
            <div className={styles.heroHeading}>
              <div className={clsx(styles.badge, isDark && styles.badgeDark)}>
                🎉 Now available on crates.io
              </div>
              <h1 className={clsx(styles.title, isDark && styles.titleDark)}>
                gh-templates
              </h1>
              <p className={clsx(styles.subtitle, isDark && styles.subtitleDark)}>
                The fastest way to create professional GitHub templates for issues, PRs, licenses, and more
              </p>
            </div>
            {/* Interactive Terminal */}
            <div className={styles.terminalContainer}>
              <div className={styles.terminal}>
                <div className={styles.terminalHeader}>
                  <div className={styles.terminalButtons}>
                    <div className={styles.terminalButtonRed} />
                    <div className={styles.terminalButtonYellow} />
                    <div className={styles.terminalButtonGreen} />
                  </div>
                  <div className={styles.terminalTitle}>Terminal</div>
                </div>
                <div className={styles.terminalContent}>
                  <div className={styles.terminalLine}>
                    <span className={styles.terminalPrompt}>$</span>
                    <span className={styles.terminalText}>
                      {typedText}
                      <span className={styles.terminalCursor} />
                    </span>
                  </div>
                </div>
              </div>
            </div>
            {/* CTA Buttons */}
            <div className={styles.ctaButtons}>
              <a href="/installation" className={styles.primaryButton}>
                <span>🚀</span>
                Get Started
              </a>
              <a href="/intro" className={clsx(styles.secondaryButton, isDark && styles.secondaryButtonDark)}>
                <span>📖</span>
                View Documentation
              </a>
              <a
                href="https://github.com/rafaeljohn9/gh-templates"
                className={clsx(styles.secondaryButton, isDark && styles.secondaryButtonDark)}
                target="_blank"
                rel="noopener noreferrer"
              >
                <span>⭐</span>
                GitHub
              </a>
            </div>
          </div>
        </div>
      </div>

      {/* Installation Section */}
      <div className={clsx(styles.installationSection, isDark && styles.installationSectionDark)}>
        <div className={styles.sectionContent}>
          <h2 className={clsx(styles.sectionTitle, isDark && styles.sectionTitleDark)}>
            Quick Installation
          </h2>
          <div className={styles.installationGrid}>
            <div className={clsx(styles.installationCard, isDark && styles.installationCardDark)}>
              <h3 className={clsx(styles.cardTitle, isDark && styles.cardTitleDark)}>
                📦 Via Cargo
              </h3>
              <div className={styles.codeBlock}>
                <code className={styles.code}>cargo install gh-templates</code>
              </div>
            </div>
            <div className={clsx(styles.installationCard, isDark && styles.installationCardDark)}>
              <h3 className={clsx(styles.cardTitle, isDark && styles.cardTitleDark)}>
                🍺 Via Homebrew
              </h3>
              <div className={styles.codeBlock}>
                <code className={styles.code}>brew install gh-templates</code>
              </div>
            </div>
            <div className={clsx(styles.installationCard, isDark && styles.installationCardDark)}>
              <h3 className={clsx(styles.cardTitle, isDark && styles.cardTitleDark)}>
                🐍 Via PyPI
              </h3>
              <div className={styles.codeBlock}>
                <code className={styles.code}>pip install gh-templates</code>
              </div>
            </div>
            <div className={clsx(styles.installationCard, isDark && styles.installationCardDark)}>
              <h3 className={clsx(styles.cardTitle, isDark && styles.cardTitleDark)}>
                📦 Via npm
              </h3>
              <div className={styles.codeBlock}>
                <code className={styles.code}>npm install -g gh-templates</code>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Features Grid */}
      <div className={styles.featuresSection}>
        <div className={styles.sectionContent}>
          <div className={styles.featuresHeader}>
            <h2 className={clsx(styles.featuresTitle, isDark && styles.featuresTitleDark)}>
              Why Developers ❤️ gh-templates
            </h2>
            <p className={clsx(styles.featuresSubtitle, isDark && styles.featuresSubtitleDark)}>
              Built by developers, for developers. Every feature designed to save you time and effort.
            </p>
          </div>
          <div className={styles.featuresGrid}>
            {features.map((feature, index) => (
              <div
                key={index}
                className={clsx(styles.featureCard, isDark && styles.featureCardDark)}
              >
                <div className={styles.featureIcon}>
                  {feature.icon}
                </div>
                <h3 className={clsx(styles.featureTitle, isDark && styles.featureTitleDark)}>
                  {feature.title}
                </h3>
                <p className={clsx(styles.featureDescription, isDark && styles.featureDescriptionDark)}>
                  {feature.description}
                </p>
                <div className={clsx(styles.featureHighlight, isDark && styles.featureHighlightDark)}>
                  {feature.highlight}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Stats Section */}
      <div className={styles.statsSection}>
        <div className={styles.sectionContent}>
          <div className={styles.statsGrid}>
            <div className={styles.statItem}>
              <div className={styles.statNumber}>50+</div>
              <div className={styles.statLabel}>Templates Supported</div>
            </div>
            <div className={styles.statItem}>
              <div className={styles.statNumber}>100+</div>
              <div className={styles.statLabel}>Downloads</div>
            </div>
            <div className={styles.statItem}>
              <div className={styles.statNumber}>10+</div>
              <div className={styles.statLabel}>GitHub Stars</div>
            </div>
            <div className={styles.statItem}>
              <div className={styles.statNumber}>99%</div>
              <div className={styles.statLabel}>Developer Satisfaction</div>
            </div>
          </div>
        </div>
      </div>

      {/* Final CTA */}
      <div className={clsx(styles.finalCtaSection, isDark && styles.finalCtaSectionDark)}>
        <div className={styles.finalCtaContent}>
          <h2 className={clsx(styles.finalCtaTitle, isDark && styles.finalCtaTitleDark)}>
            Ready to streamline your GitHub workflow?
          </h2>
          <p className={clsx(styles.finalCtaSubtitle, isDark && styles.finalCtaSubtitleDark)}>
            Join developers who save hours every week with gh-templates
          </p>
          <div className={styles.finalCtaButtons}>
            <a href="/installation" className={styles.finalPrimaryButton}>
              Start Building →
            </a>
            <a href="/intro" className={clsx(styles.finalSecondaryButton, isDark && styles.finalSecondaryButtonDark)}>
              Read the Docs
            </a>
          </div>
        </div>
      </div>
    </div >
  );
}