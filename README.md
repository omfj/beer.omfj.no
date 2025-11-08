# 🍺 Beer Counter

A social drinking tracking application that celebrates beer culture while welcoming all beverage enthusiasts!

## About

Beer Counter is a fun, competitive app designed to track drinks at social gatherings and events. While our heart belongs to the timeless tradition of beer drinking, we embrace all drink preferences in a spirit of inclusivity and good times.

### Why Beer? 🍻

Beer has been bringing people together for thousands of years. From ancient civilizations to modern craft breweries, beer represents:

- **Community**: Sharing a beer has always been about connection and camaraderie
- **Tradition**: Beer culture is rich with history, craftsmanship, and regional pride
- **Variety**: From pilsners to IPAs, stouts to sours - there's a beer for every palate
- **Moderation**: Beer's lower ABV makes it ideal for social drinking

That's why Beer Counter gives a small bonus (10%) to beer registrations - to honor this classic beverage while keeping competition fun!

### Inclusive by Design

While we celebrate beer, we respect everyone's preferences:

- Track **any** drink type: wine, cocktails, shots, cider, and more
- All beverages earn points based on volume and alcohol content
- Fair scoring system that works for everyone
- No judgment, just friendly competition

## Features

- 🎉 **Event Creation**: Set up drinking events for parties, gatherings, or competitions
- 📸 **Photo Registration**: Snap a pic of your drink to register it
- 🏆 **Real-time Leaderboards**: See who's leading with live WebSocket updates
- 📊 **Smart Scoring**: Points calculated based on volume and ABV (with a beer bonus!)
- 🎨 **Customizable Events**: Add colors and names to personalize your events
- 📱 **Mobile-First**: Built for smartphones and social situations

## Technology Stack

This is a modern TypeScript monorepo built with:

- **Frontend**: SvelteKit with Cloudflare Pages
- **Backend**: Cloudflare Workers & Durable Objects
- **Database**: Cloudflare D1 (SQLite)
- **Storage**: Cloudflare R2 for images
- **Real-time**: WebSocket via Durable Objects
- **ORM**: Drizzle ORM

## Getting Started

### Prerequisites

- Node.js 18+
- pnpm
- Cloudflare account (for deployment)

### Installation

```bash
# Clone the repository
git clone https://github.com/omfj/beer.omfj.no.git
cd beer.omfj.no

# Install dependencies
pnpm install

# Start development servers
pnpm dev
```

### Development Commands

```bash
pnpm dev          # Start all apps in development mode
pnpm build        # Build all apps
pnpm lint         # Lint all apps
pnpm format       # Format all apps
```

See [CLAUDE.md](./CLAUDE.md) for detailed development documentation.

## Project Philosophy

Beer Counter was created to add a fun, competitive element to social drinking. We believe:

1. **Beer is awesome** - It deserves celebration and a small advantage in our scoring
2. **Choice matters** - Everyone should drink what they enjoy
3. **Fun first** - This is about good times with friends, not serious competition
4. **Responsible drinking** - Track responsibly, drink responsibly

## Contributing

Contributions are welcome! Whether you want to add features, fix bugs, or improve documentation, we'd love your help.

## License

This project is open source and available for personal use.

## Disclaimer

Please drink responsibly. This app is for entertainment purposes and should not encourage excessive drinking. Always know your limits and never drink and drive.

---

🍺 Cheers to good times and great beers! (And other drinks too! 🍷🍹🥃)
