// src/components/LockButton.js
import React from 'react';
import './lockButton.css';

const LockButton = ({ name, isLocked, toggleLock }) => {
  const handleClick = () => toggleLock(name);

  return (
    <div className={`lock-button ${isLocked ? 'locked' : 'unlocked'}`} onClick={handleClick}>
      <img src={isLocked ? 'react-interface/src/img/padlock-unlock.png' : 'react-interface/src/img/lock.png'} alt={isLocked ? 'Locked' : 'Unlocked'} />
    </div>
  );
};

export default LockButton;
