import React, { MouseEventHandler } from 'react';
import clsx from 'clsx';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import CircularProgress from '@material-ui/core/CircularProgress';
import { green } from '@material-ui/core/colors';
import Button from '@material-ui/core/Button';
import Fab from '@material-ui/core/Fab';
import CheckIcon from '@material-ui/icons/Check';
import UploadIcon from '@material-ui/icons/Folder';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
      alignItems: 'center',
    },
    wrapper: {
      margin: theme.spacing(1),
      position: 'relative',
    },
    buttonSuccess: {
      backgroundColor: green[500],
      '&:hover': {
        backgroundColor: green[700],
      },
    },
    fabProgress: {
      color: green[500],
      position: 'absolute',
      top: -6,
      left: -6,
      zIndex: 1,
    },
    buttonProgress: {
      color: green[500],
      position: 'absolute',
      top: '50%',
      left: '50%',
      marginTop: -12,
      marginLeft: -12,
    },
  }),
);

export enum ProcessTypes {
  initial,
  success,
  failed,
  cancelled,
  processing,
}

interface CircularIntegrationProps {
  processState: ProcessTypes,
  onClick: MouseEventHandler,
}

export default function CircularIntegration(props: CircularIntegrationProps) {
  const classes = useStyles();
  // const [loading, setLoading] = React.useState(false);
  // const [success, setSuccess] = React.useState(false);
  // const timer = React.useRef<number>();

  const success = props.processState === ProcessTypes.success
  const loading = props.processState === ProcessTypes.processing
  const handleButtonClick = props.onClick

  const buttonClassname = clsx({
    [classes.buttonSuccess]: success,
  });

  // React.useEffect(() => {
  //     return () => {
  //         clearTimeout(timer.current);
  //     };
  // }, []);

  // const handleButtonClick = () => {
  //     if (!loading) {
  //         setSuccess(false);
  //         setLoading(true);
  //         timer.current = window.setTimeout(() => {
  //             setSuccess(true);
  //             setLoading(false);
  //         }, 2000);
  //     }
  // };

  return (
    <div className={classes.root}>
      <div className={classes.wrapper}>
        <Fab
          aria-label="save"
          color="primary"
          className={buttonClassname}
          onClick={handleButtonClick}
        >
          {success ? <CheckIcon /> : <UploadIcon />}
        </Fab>
        {loading && <CircularProgress size={68} className={classes.fabProgress} />}
      </div>
      <div className={classes.wrapper}>
        <Button
          variant="contained"
          color="primary"
          className={buttonClassname}
          disabled={loading}
          onClick={handleButtonClick}
        >
          Upload your savegame
        </Button>
        {loading && <CircularProgress size={24} className={classes.buttonProgress} />}
      </div>
    </div>
  );
}
