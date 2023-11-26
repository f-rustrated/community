import React from 'react';
import { ComponentPreview, Previews } from '@react-buddy/ide-toolbox';
import { PaletteTree } from './palette';
import App from '../pages/App';
import { AppBar } from '../components/app/AppBar';
import SignIn from '../components/app/SignIn';

const ComponentPreviews = () => {
  return (
    <Previews palette={<PaletteTree />}>
      <ComponentPreview path='/App'>
        <App />
      </ComponentPreview>
      <ComponentPreview path='/AppBar'>
        <AppBar />
      </ComponentPreview>
      <ComponentPreview path='/SignIn'>
        <SignIn />
      </ComponentPreview>
    </Previews>
  );
};

export default ComponentPreviews;