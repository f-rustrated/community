import * as React from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import logo from '../../assets/logo.svg';
import SignIn from './SignIn';

export default function SignInDialog() {
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <React.Fragment>
      <Button  onClick={handleClickOpen}>
        <img src={logo} className='App-logo' alt='logo' />
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <SignIn></SignIn>
      </Dialog>
    </React.Fragment>
  );
}