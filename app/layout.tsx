import type {Metadata} from 'next';
import { Inter, JetBrains_Mono } from 'next/font/google';
import './globals.css';

const inter = Inter({
  subsets: ['latin'],
  variable: '--font-sans',
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-mono',
});

export const metadata: Metadata = {
  title: 'MPL Technomancy OS',
  description: 'Magick Programming Language Framework. Compiler, OVM, and IDE for non-linear probabilistic computation.',
};

export default function RootLayout({children}: {children: React.ReactNode}) {
  return (
    <html lang="en" className={`${inter.variable} ${jetbrainsMono.variable} dark`} suppressHydrationWarning>
      <body className="bg-[#050505] text-[#e0e0e0] font-mono antialiased" suppressHydrationWarning>
        {children}
      </body>
    </html>
  );
}
