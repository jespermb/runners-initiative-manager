// Components
import Home from './pages/Home.svelte'
import Campaign from './pages/Campaign.svelte'
import NotFound from './pages/NotFound.svelte'
import Encounter from './pages/Encounter.svelte'

// Export the route definition object
export default {
    // Exact path
    '/': Home,
    '/campaign/:id': Campaign,
    '/encounter/:id': Encounter,
    // Catch-all, must be last
    '*': NotFound,
}