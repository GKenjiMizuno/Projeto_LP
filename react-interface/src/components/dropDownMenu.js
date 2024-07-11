import React, { useState, useEffect, useRef } from 'react';
import './dropDownMenu.css';

const DropdownMenu = ({ handleModeChange }) => {
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef(null);

  const handleToggle = () => {
    setIsOpen(!isOpen);
  };

  const handleClickOutside = (event) => {
    if (menuRef.current && !menuRef.current.contains(event.target)) {
      setIsOpen(false);
    }
  };

  useEffect(() => {
    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div className="dropdown">
      <button className="dropdown-btn"onClick={handleToggle}>ATIVAR <br></br>MODO</button>
      {isOpen && (
        <div className="menu" ref={menuRef}>
          <button onClick={() => handleModeChange('dormir')}>Dormir</button>
          <button onClick={() => handleModeChange('acordar')}>Acordar</button>
          <button onClick={() => handleModeChange('limpar')}>Limpar casa</button>
          <button onClick={() => handleModeChange('trancar')}>Trancar casa</button>
          <button onClick={() => handleModeChange('destrancar')}>Destrancar casa</button>
          <button onClick={() => handleModeChange('filme')}>Assistir filme</button>
          <button onClick={() => handleModeChange('musica')}>Ouvir música</button>
        </div>
      )}
    </div>
  );
};

export default DropdownMenu;
