import React from 'react';
import Sidebar from "./Sidebar"
import Navbar from "./Navbar";
import Section from "./Section";
import Table from "./Table";
import '../tellerDashboard/Dashboard.css'
function Dashboard(){


    return(
 <div className="home">
 <Sidebar/>
 <div className='homeContainer'>
    <Navbar/>
    <div className='widgets'>
        <Section content="Queue length"/>
        <Section content="Average waiting time"/>
        <Section content="Average service Time"/>
    </div>
    <div className="Ttable">
        <Table/>
    </div>
 </div>
 </div>
    )
}

export default Dashboard;