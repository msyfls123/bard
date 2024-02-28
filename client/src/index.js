import './css/index.styl'

import wasm from '../Cargo.toml';

const cos = new COS({
  getAuthorization: async (options, callback) => {
    const url = 'https://void.ebichu.cc/sts';
    let data;
    try {
      data = await fetch(url).then(res => res.json());
    } catch (err) {
      console.err('connect secret server error', err)
    }
    const { credentials, startTime, expiredTime } = data || {};
    if (!(data && credentials)) {
      return console.error('data invalid:\n' + JSON.stringify(data, null, 2))
    };
    callback({
      TmpSecretId: credentials.tmpSecretId,
      TmpSecretKey: credentials.tmpSecretKey,
      SecurityToken: credentials.sessionToken,
      StartTime: startTime,
      ExpiredTime: expiredTime,
    });
  }
});

function handleFileChange(e) {
  const file = e.target.files[0];
  if (!file) {
    return Promise.reject('no file');
  }
  const name = file.name;
  return new Promise((resolve, reject) => {
    cos.putObject(
      {
        Bucket: 'kimi-1251502833',
        Region: 'ap-beijing',
        Key: `cos-test/${name}`,
        Body: file,
        onProgress: function(progressData) {
          console.log('percentage', JSON.stringify(progressData));
        }
      },
      function(err, data) {
        if (err) {
          reject(err);
          console.error('upload fail', err);
        } else {
          resolve(data);
          console.info('upload success', data);
        }
      },
    );
  })
}

function listFolder() {
  return new Promise((resolve, reject) => {
    cos.getBucket({
      Bucket: 'kimi-1251502833', /* 填入您自己的存储桶，必须字段 */
      Region: 'ap-beijing',  /* 存储桶所在地域，例如ap-beijing，必须字段 */
      Prefix: 'cos-test/',              /* Prefix表示列出的object的key以prefix开始，非必须 */
      Delimiter: '/',            /* Deliter表示分隔符, 设置为/表示列出当前目录下的object, 设置为空表示列出所有的object，非必须 */
    }, function(err, data) {
      if (err) {
        reject(err);
        console.error('list folder fail', err);
      } else {
        resolve(data);
        console.info('list folder success', data);
      }
    });
  
  })
}

window.listFolder = listFolder

wasm().then(({ run_app }) => {
  run_app({
    handleFileChange,
    user: window.app.user,
  });
});
