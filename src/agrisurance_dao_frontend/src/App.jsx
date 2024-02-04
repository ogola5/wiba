import React from 'react';
import UserProfileForm from './components/UserProfileComponent/Profile.js'; // Adjust the path according to your file structure
import GovernanceProposalForm from './components/GovernanceProposalComponent/GovernanceProposalForm.js';

const App = () => {
  return (
    <div>
      <h1>Welcome to the User Profile Application</h1>
      <UserProfileForm />
      <GovernanceProposalForm/>
    </div>
  );
};

export default App;
//src/agrisurance_dao_frontend/src/components/UserProfileComponent/Profile.js