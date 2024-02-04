import React, { useState } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as backend_idl, canisterId as backend_id } from '../../../../declarations/agrisurance_dao_backend';


const agent = new HttpAgent();
const backend = Actor.createActor(backend_idl, { agent, canisterId: backend_id });

const Profile = () => {
  const [name, setName] = useState('');
  const [role, setRole] = useState('Farmer'); // Assuming 'Farmer' is a valid role
  const [stakeInDao, setStakeInDao] = useState('');
  const [message, setMessage] = useState('');

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      // Convert role to the correct variant format
      const userProfile = await backend.create_user_profile(name, { [role]: null }, Number(stakeInDao));
      if (userProfile) {
        setMessage('User profile created successfully!');
      } else {
        setMessage('Failed to create user profile.');
      }
    } catch (error) {
      setMessage(`Error: ${error.message}`);
    }
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Name"
        />
        <select value={role} onChange={(e) => setRole(e.target.value)}>
          <option value="Farmer">Farmer</option>
          <option value="Consumer">Consumer</option>
        </select>
        <input
          type="number"
          value={stakeInDao}
          onChange={(e) => setStakeInDao(e.target.value)}
          placeholder="Stake in DAO"
        />
        <button type="submit">Create Profile</button>
      </form>
      {message && <p>{message}</p>}
    </div>
  );
};

export default Profile;
