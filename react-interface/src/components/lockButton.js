// src/components/LockButton.js
import React,{useState} from 'react';
import './lockButton.css';

const LockButton = ({ name, isLocked, toggleLock }) => {
  const handleClick = () => {
    toggleLock(name);
    toggleSVG();
  }
  const [showSvg, setshowSvg] = useState(true);

  // Função para alternar os SVGs
  const toggleSVG = () => {
    setshowSvg(!showSvg);
  };
  return (
    
    <div className={`lock-button ${isLocked ? 'locked' : 'unlocked'}`} onClick={handleClick}>
      <svg style={{ opacity: showSvg ? 1 : 0 }} id="unlock-svg"xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-unlock" viewBox="0 0 16 16">
        <path d="M11 1a2 2 0 0 0-2 2v4a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h5V3a3 3 0 0 1 6 0v4a.5.5 0 0 1-1 0V3a2 2 0 0 0-2-2M3 8a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V9a1 1 0 0 0-1-1z"/>
      </svg>
      <svg style={{ opacity: showSvg ? 0 : 1 }} id="lock-svg" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-lock" viewBox="0 0 16 16">
        <path d="M8 1a2 2 0 0 1 2 2v4H6V3a2 2 0 0 1 2-2m3 6V3a3 3 0 0 0-6 0v4a2 2 0 0 0-2 2v5a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2M5 8h6a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1"/>
      </svg>
    </div>
  );
};

export default LockButton;
