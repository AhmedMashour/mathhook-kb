import type { Config } from 'tailwindcss'

/**
 * MathHook Brand Colors Tailwind Configuration
 *
 * Brand Identity:
 * - The Sage (Educational) + The Engineer (Performance)
 * - Tagline: "Symbolic Power. Educational Clarity. Native Speed."
 *
 * Color Palette:
 * - Logic Navy (#0F172A): Deep academic depth, dark mode friendly
 * - Rust Core (#E64524): Primary accent, Rust language homage
 * - Solve Cyan (#06B6D4): Speed, SIMD, calculated solutions
 * - Chalk White (#F8FAFC): High contrast for math/formulas
 * - Step Green (#10B981): Success, step-by-step explanations
 */

export default <Config>{
  content: [
    './components/**/*.{js,vue,ts}',
    './layouts/**/*.vue',
    './pages/**/*.vue',
    './plugins/**/*.{js,ts}',
    './nuxt.config.{js,ts}',
    './app.vue',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // === MathHook Brand Colors ===

        // Primary: Logic Navy - Deep, serious, academic
        'logic-navy': {
          DEFAULT: '#0F172A',
          50: '#E8EBF0',
          100: '#CDD2DC',
          200: '#9BA5B9',
          300: '#697896',
          400: '#3E4D6E',
          500: '#1E293B',
          600: '#172033',
          700: '#111827',
          800: '#0C1220',
          900: '#0F172A',
          950: '#080C15',
        },

        // Accent: Rust Core - Energy, Rust language, critical elements
        'rust-core': {
          DEFAULT: '#E64524',
          50: '#FEF2EF',
          100: '#FDE6E0',
          200: '#FBCCC0',
          300: '#F8A691',
          400: '#F47A5C',
          500: '#E64524',
          600: '#C9351A',
          700: '#A62B15',
          800: '#872414',
          900: '#6F2016',
          950: '#3C0C08',
        },

        // Secondary: Solve Cyan - Speed, solutions, SIMD
        'solve-cyan': {
          DEFAULT: '#06B6D4',
          50: '#ECFEFF',
          100: '#CFFAFE',
          200: '#A5F3FC',
          300: '#67E8F9',
          400: '#22D3EE',
          500: '#06B6D4',
          600: '#0891B2',
          700: '#0E7490',
          800: '#155E75',
          900: '#164E63',
          950: '#083344',
        },

        // Text: Chalk White - Formulas, high contrast
        'chalk': {
          DEFAULT: '#F8FAFC',
          50: '#FFFFFF',
          100: '#F8FAFC',
          200: '#F1F5F9',
          300: '#E2E8F0',
          400: '#CBD5E1',
          500: '#94A3B8',
          600: '#64748B',
          700: '#475569',
          800: '#334155',
          900: '#1E293B',
        },

        // Success: Step Green - Educational steps, success states
        'step-green': {
          DEFAULT: '#10B981',
          50: '#ECFDF5',
          100: '#D1FAE5',
          200: '#A7F3D0',
          300: '#6EE7B7',
          400: '#34D399',
          500: '#10B981',
          600: '#059669',
          700: '#047857',
          800: '#065F46',
          900: '#064E3B',
          950: '#022C22',
        },

        // === Semantic Aliases ===
        primary: {
          DEFAULT: '#E64524',
          light: '#F47A5C',
          dark: '#A62B15',
        },
        secondary: {
          DEFAULT: '#06B6D4',
          light: '#22D3EE',
          dark: '#0E7490',
        },
        success: {
          DEFAULT: '#10B981',
          light: '#34D399',
          dark: '#047857',
        },
        background: {
          DEFAULT: '#0F172A',
          light: '#F8FAFC',
          card: '#1E293B',
        },
      },

      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        heading: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', 'monospace'],
      },

      fontSize: {
        // Display sizes for hero sections
        'display-xl': ['5rem', { lineHeight: '1.1', fontWeight: '800' }],
        'display-lg': ['4rem', { lineHeight: '1.1', fontWeight: '800' }],
        'display': ['3rem', { lineHeight: '1.2', fontWeight: '700' }],
      },

      backgroundImage: {
        // Brand gradients
        'gradient-brand': 'linear-gradient(135deg, #E64524 0%, #06B6D4 100%)',
        'gradient-hero': 'linear-gradient(135deg, #E64524 0%, #F47A5C 50%, #06B6D4 100%)',
        'gradient-navy': 'linear-gradient(180deg, #0F172A 0%, #1E293B 100%)',
        'gradient-glow': 'radial-gradient(ellipse at center, rgba(6, 182, 212, 0.15) 0%, transparent 70%)',
        'gradient-rust-glow': 'radial-gradient(ellipse at center, rgba(230, 69, 36, 0.15) 0%, transparent 70%)',
      },

      boxShadow: {
        'glow-cyan': '0 0 40px rgba(6, 182, 212, 0.3)',
        'glow-rust': '0 0 40px rgba(230, 69, 36, 0.3)',
        'glow-green': '0 0 40px rgba(16, 185, 129, 0.3)',
        'card': '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
        'card-hover': '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
      },

      animation: {
        'blob': 'blob 7s infinite',
        'fade-in-up': 'fade-in-up 1s ease-out forwards',
        'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'float': 'float 6s ease-in-out infinite',
      },

      keyframes: {
        blob: {
          '0%': { transform: 'translate(0px, 0px) scale(1)' },
          '33%': { transform: 'translate(30px, -50px) scale(1.1)' },
          '66%': { transform: 'translate(-20px, 20px) scale(0.9)' },
          '100%': { transform: 'translate(0px, 0px) scale(1)' },
        },
        'fade-in-up': {
          '0%': { opacity: '0', transform: 'translateY(30px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-20px)' },
        },
      },

      borderRadius: {
        'xl': '0.75rem',
        '2xl': '1rem',
        '3xl': '1.5rem',
      },

      typography: {
        DEFAULT: {
          css: {
            '--tw-prose-body': '#CBD5E1',
            '--tw-prose-headings': '#F8FAFC',
            '--tw-prose-links': '#06B6D4',
            '--tw-prose-code': '#E64524',
            'code::before': { content: '""' },
            'code::after': { content: '""' },
          },
        },
      },
    },
  },
  plugins: [],
}
