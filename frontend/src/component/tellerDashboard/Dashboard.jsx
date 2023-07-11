import React from 'react';
import Sidebar from "./Sidebar"
import Navbar from "./Navbar";
import Section from "./Section";
import Section2 from "./Section2";
import Section3 from "./Section3";
import Table from "./Table";
import '../tellerDashboard/Dashboard.css'
function Dashboard(){


    return(
 <div className="home">
 <Sidebar/>
 <div className='homeContainer'>
    <Navbar/>
    <div className='widgets'>
        <Section content="Average waiting time"/>
        <Section2/>
        <Section3/>
    </div>
    <div className="Ttable">
        <Table/>
    </div>
 </div>
 </div>
    )
}

export default Dashboard;