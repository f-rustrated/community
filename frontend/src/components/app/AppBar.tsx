import React from 'react';
import SignInDialog from './SignInDialog';

export function AppBar() {

  const styles = {
    logo: {
      margin: '4px',
    },
  };

  return (
    <div>
      <SignInDialog></SignInDialog>
    </div>
  );
}