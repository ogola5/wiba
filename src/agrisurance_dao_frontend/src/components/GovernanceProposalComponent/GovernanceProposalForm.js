import React, { useState } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as backend_idl, canisterId as backend_id } from '../../../../declarations/agrisurance_dao_backend';

// Initialize the actor to interact with the ICP backend
const agent = new HttpAgent();
const backend = Actor.createActor(backend_idl, { agent, canisterId: backend_id });

const GovernanceProposalForm = () => {
  const [proposalDetails, setProposalDetails] = useState('');
  const [proposerId, setProposerId] = useState(''); // This should be derived from the user's identity
  const [message, setMessage] = useState('');

  // Since VotingRecord is complex, you might manage it separately
  // For the sake of this example, we'll assume it's an empty array
  const votingRecords = [];

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      // Make sure proposerId is a number (u64)
      const numericProposerId = BigInt(proposerId);
      const governanceProposal = await backend.create_governance_proposal(proposalDetails, numericProposerId, votingRecords);
      if (governanceProposal) {
        setMessage('Governance proposal created successfully!');
      } else {
        setMessage('Failed to create governance proposal.');
      }
    } catch (error) {
      setMessage(`Error: ${error.message}`);
    }
  };

  return (
    <div>
      <h1>Create Governance Proposal</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Proposal Details:
            <textarea
              value={proposalDetails}
              onChange={(e) => setProposalDetails(e.target.value)}
              placeholder="Enter the details of the proposal"
            />
          </label>
        </div>
        <div>
          <label>
            Proposer ID:
            <input
              type="text"
              value={proposerId}
              onChange={(e) => setProposerId(e.target.value)}
              placeholder="Enter your proposer ID"
            />
          </label>
        </div>
        {/* Other fields related to VotingRecord would go here */}
        <button type="submit">Submit Proposal</button>
      </form>
      {message && <p>{message}</p>}
    </div>
  );
};

export default GovernanceProposalForm;
