// Components
import Home from './pages/Home.svelte'
import Campaign from './pages/Campaign.svelte'
import NotFound from './pages/NotFound.svelte'

// Export the route definition object
export default {
    // Exact path
    '/': Home,

    // Using named parameters, with last being optional
    '/campaign/:id': Campaign,

    // Catch-all, must be last
    '*': NotFound,
}