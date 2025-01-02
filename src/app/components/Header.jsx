import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import newFile from '/src/app/assets/new-file.svg';
import openFile from '/src/app/assets/open-file.svg';
function Header() {
  function handleClick() {
    invoke("my_custom_command")
      .then((message) => console.log(message))
      .catch((error) => console.error(error));
  }
  return (
    <header>
      <div className="svg-group">
        <div className="newFileButtonContainer">
          <img src={newFile} alt="Description" className="newFileButtonImage"/>
        </div>
        <div className="openFileButtonContainer">
          <img src={openFile} alt="Description" className="openFileButton"/>
        </div>
      </div>
    </header>
    
  );
}

export default Header;
