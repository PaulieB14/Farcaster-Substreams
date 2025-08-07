# Farcaster Statistics Substream

A comprehensive substream that tracks valuable statistics from Farcaster's core contracts on Optimism.

## 🎯 Overview

This substream monitors all three core Farcaster contracts on Optimism to provide real-time analytics and insights:

- **IdRegistry** (0x00000000fc6c5f01fc30151999387bb99a9f489b) - User registrations and identity management
- **StorageRegistry** (0x00000000fcce7f938e7ae6d3c335bd6a1a7c593d) - Posts/casts and storage management
- **KeyRegistry** (0x00000000fc1237824fb747abde0ff18990e59b7e) - Signing key management

## 📊 What We Track

### User Metrics
- **Total Users**: Cumulative user registrations
- **Daily Active Users**: Users active in the last 24 hours
- **User Growth Rate**: New registrations over time
- **User Distribution**: Geographic and temporal patterns

### Content Metrics
- **Total Posts**: All posts/casts created
- **Daily Posts**: Posts created in the last 24 hours
- **Post Types**: Breakdown of casts, replies, and recasts
- **Content Engagement**: Reactions, replies, and recasts per post

### Engagement Metrics
- **Reactions**: Likes and other reactions
- **Follows**: Follow/unfollow events
- **Engagement Rate**: Average engagement per user
- **Viral Content**: Posts with high engagement

### Storage Metrics
- **Storage Units**: Purchased and used storage
- **Storage Economics**: Cost and usage patterns
- **Storage Transfers**: Storage unit transfers between users

### Security Metrics
- **Key Changes**: Signing key rotations
- **Authentication Events**: Security-related activities
- **Account Recovery**: Recovery address changes

## 🏆 Top Performers

### Top Users
- **By Post Count**: Most active content creators
- **By Follower Count**: Most followed users
- **By Engagement**: Users with highest engagement rates

### Top Posts
- **By Reactions**: Most liked posts
- **By Replies**: Most discussed posts
- **By Recasts**: Most shared posts

## 📈 Analytics Features

### Real-time Metrics
- Live tracking of all Farcaster activity
- Instant updates on user growth and engagement
- Real-time monitoring of network health

### Historical Analysis
- Trend analysis over time
- Seasonal patterns and growth cycles
- Comparative analysis across time periods

### Predictive Insights
- User growth projections
- Engagement trend predictions
- Content performance forecasting

## 🚀 Use Cases

### For Developers
- **API Integration**: Real-time data for applications
- **Analytics Dashboards**: Comprehensive Farcaster analytics
- **Bot Development**: Data for intelligent bots and tools

### For Researchers
- **Social Network Analysis**: Study of Farcaster's network structure
- **User Behavior Research**: Understanding user engagement patterns
- **Economic Analysis**: Storage unit economics and tokenomics

### For Investors
- **Network Growth Tracking**: Monitor Farcaster's adoption
- **Engagement Metrics**: Assess platform health and activity
- **Competitive Analysis**: Compare with other social platforms

## 🔧 Technical Details

### Data Schema
The substream outputs structured data including:
- User registration events with timestamps
- Post creation events with metadata
- Reaction and engagement events
- Storage and key management events

### Performance
- **Low Latency**: Real-time processing of blockchain events
- **High Throughput**: Handles all Farcaster activity on Optimism
- **Reliable**: Built on Substreams' robust infrastructure

### Scalability
- **Horizontal Scaling**: Automatically scales with network growth
- **Efficient Processing**: Optimized for high-volume event processing
- **Cost Effective**: Minimal resource requirements

## 📋 Installation & Usage

### Prerequisites
- Substreams CLI installed
- Access to Optimism mainnet

### Quick Start
```bash
# Install the substream
substreams install farcaster_stats

# Run locally for testing
substreams run map_farcaster_events

# Consume the substream
substreams consume farcaster_stats
```

### API Integration
```javascript
// Example: Consume Farcaster events
const substream = new Substreams('farcaster_stats');
substream.on('data', (event) => {
  console.log('New Farcaster event:', event);
});
```

## 🎨 Visualization Examples

### Dashboard Metrics
- **User Growth Chart**: Cumulative user registrations over time
- **Activity Heatmap**: Daily posting patterns
- **Engagement Funnel**: User journey from registration to active posting
- **Top Content**: Most engaging posts and users

### Real-time Widgets
- **Live User Counter**: Real-time user count
- **Recent Activity Feed**: Latest posts and reactions
- **Network Health**: Current engagement and activity levels

## 🔮 Future Enhancements

### Planned Features
- **Sentiment Analysis**: Content sentiment tracking
- **Influence Scoring**: User influence metrics
- **Content Categorization**: Automatic content classification
- **Cross-chain Analysis**: Integration with other networks

### Advanced Analytics
- **Network Effects**: Study of viral content spread
- **User Cohorts**: Cohort analysis for user retention
- **Predictive Modeling**: ML-powered engagement predictions

## 🤝 Contributing

We welcome contributions! Areas for improvement:
- Enhanced event parsing
- Additional analytics metrics
- Performance optimizations
- Documentation improvements

## 📄 License

MIT License - see LICENSE file for details.

## 🔗 Links

- **Substreams.dev**: [Package Page](https://substreams.dev/packages/farcaster-stats)
- **Documentation**: [Farcaster Docs](https://docs.farcaster.xyz/)
- **GitHub**: [Repository](https://github.com/your-username/farcaster-substream)
- **Discord**: [Community](https://discord.gg/farcaster)

---

**Built with ❤️ for the Farcaster community** 