var ghpages = require('gh-pages');

ghpages.publish(
    'public', // path to public directory
    {
      branch: 'gh-pages',
      repo: 'https://github.com/SilenLoc/home.git', // Update to point to your repository
      user: {
        name: 'SilenLoc', // update to use your name
        email: 'silen.locatelli@gmx.ch' // Update to use your email
      }
    },
    () => {
      console.log('Deploy Complete!')
    }
)