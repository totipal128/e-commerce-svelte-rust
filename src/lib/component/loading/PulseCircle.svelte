<div class="pulse"></div>

<style>
  .pulse {
    width: 20px;
    height: 20px;
    background: #3b82f6;
    border-radius: 50%;
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% {
      transform: scale(1);
      opacity: 1;
    }
    100% {
      transform: scale(2);
      opacity: 0;
    }
  }
</style>
