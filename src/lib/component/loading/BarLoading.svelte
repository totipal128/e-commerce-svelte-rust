<div class="bars">
  <div></div>
  <div></div>
  <div></div>
</div>

<style>
  .bars div {
    width: 6px;
    height: 30px;
    background: #3b82f6;
    margin: 0 2px;
    display: inline-block;
    animation: stretch 1.2s infinite ease-in-out;
  }

  .bars div:nth-child(2) {
    animation-delay: -1.1s;
  }
  .bars div:nth-child(3) {
    animation-delay: -1s;
  }

  @keyframes stretch {
    0%,
    40%,
    100% {
      transform: scaleY(0.4);
    }
    20% {
      transform: scaleY(1);
    }
  }
</style>
