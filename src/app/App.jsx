import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import "./styles/App.css";
// import App from "./App";
// import {Footer, Header, LeftNav, RightNav, MainContent} from "./components"
import Header from "./components/Header"
import LeftNav from "./components/LeftNav"
import MainContent from "./components/MainContent"
import Footer from "./components/Footer"

function App() {
  return (
    <main className="container">
        <Header/>
        <LeftNav/>
        <MainContent/>
        <Footer/>
    </main>
  );
}

export default App;
