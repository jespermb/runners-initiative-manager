<script>
  // Generate random delays for animations
  const scanDelay = Math.random() * 3; // Random delay between 0-3 seconds
  const glitchDelay = Math.random() * 2 + 1; // Random delay between 1-3 seconds
</script>

<header class="cyberpunk-header">
  <div class="glitch-container">
    <h1 class="cyberpunk-title" style="--scan-delay: {scanDelay}">Runners initiative manager</h1>
    <h1 class="cyberpunk-title glitch" style="--glitch-delay: {glitchDelay}" aria-hidden="true">Runners initiative manager</h1>
  </div>
</header>

<style>
  .cyberpunk-header {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0.5rem 0;
    position: relative;
    overflow: hidden;
    border-bottom: 2px solid var(--neon-border-color);
    box-shadow: 
      0 4px 8px -2px rgba(0, 243, 255, 0.7),
      0 0 15px rgba(0, 243, 255, 0.5);
    margin: 0 -1rem 1rem -1rem; /* Negative margin to extend to edges */
  }

  .glitch-container {
    position: relative;
    width: 100%;
    text-align: center;
  }

  .cyberpunk-title {
    text-transform: uppercase;
    font-size: 1rem;
    font-weight: 800;
    letter-spacing: 1px;
    padding: 0.5rem 0;
    color: white;
    position: relative;
    background-color: transparent;
    text-shadow:
      0 0 2px var(--neon-text-color),
      0 0 4px var(--neon-text-color);
    white-space: nowrap;
  }

  .cyberpunk-title::before {
    content: '';
    position: absolute;
    top: 0;
    left: -10px;
    width: 10px;
    height: 100%;
    background-color: var(--neon-border-color);
    filter: blur(5px);
    animation: scan 8s linear infinite;
    animation-delay: calc(var(--scan-delay, 0) * 1s);
  }

  .cyberpunk-title.glitch {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    opacity: 0.5;
    clip-path: polygon(0 0, 100% 0, 100% 45%, 0 45%);
    transform: translate(-2px, 0);
    animation: glitch-top 5s infinite alternate-reverse;
    animation-delay: calc(var(--glitch-delay, 0) * 1s);
  }

  @keyframes scan {
    0% {
      left: -10px;
    }
    100% {
      left: 100%;
    }
  }

  @keyframes glitch-top {
    0%, 90% {
      transform: translate(-1px, 0);
    }
    10%, 30%, 50%, 70% {
      transform: translate(1px, 0);
    }
    20%, 40%, 60%, 80%, 100% {
      transform: translate(-1px, 0);
    }
  }
</style>
