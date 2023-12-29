# ---
# jupyter:
#   jupytext:
#     formats: ipynb,py:light
#     text_representation:
#       extension: .py
#       format_name: light
#       format_version: '1.5'
#       jupytext_version: 1.14.7
#   kernelspec:
#     display_name: Python 3 (ipykernel)
#     language: python
#     name: python3
# ---

import numpy as np

# ## Checking the method

pis = [
[19, 13, 30], 
[18, 19, 22], 
[20, 25, 34], 
[12, 31, 28], 
[20, 19, 15], 
]
vis = [
[-2,  1, -2],
[-1, -1, -2],
[-2, -2, -4],
[-1, -2, -1],
[ 1, -5, -3],
]

units = np.eye(3).tolist()

a_raw = [ 
    [np.cross(vi, u).tolist() for u in units]+[np.cross(u, pi).tolist() for u in units]  
    for (vi, pi) in zip(vis, pis)
];
a_raw

a2 = np.asarray(a_raw).transpose([0,2,1])
ad = (a2[1:]-a2[0]).reshape([-1,6])


vipi = np.asarray([np.cross(vi, pi) for (vi, pi) in zip(vis, pis)])
bd = (vipi[1:]-vipi[0]).reshape([-1])

np.linalg.lstsq(ad, bd, rcond=None)

# ## Part 2 -- getting around numerical issues

# +
qra = np.asarray(
[[63289504, 24132, -16265470, 22278092605879322373, -992105642873967636, -1038240233728460352],
 [24132, 59030179, 4693860, -842543887384163432, 19228820341221620488, -335844856985045875],
 [-16265470, 4693860, 21771727, -5594304273593480159, 1181417637884256190, 6710911521040139461],
 [22278092605879322373, -842543887384163432, -5594304273593480159, 9640066534655746190194235473368, 142045301814662562043958825438, -1100034995787147297839940703996],
 [-992105642873967636, 19228820341221620488, 1181417637884256190, 142045301814662562043958825438, 10140514586741622580198991057677, -52797591706152780183481331437],
 [-1038240233728460352, -335844856985045875, 6710911521040139461, -1100034995787147297839940703996, -52797591706152780183481331437, 6170048379161753749790069486243]])

qrb = np.asarray([[11473108191929194810390],
 [19433434162164456740313],
 [3350558080824623994526],
 [4191135996134993361810603521471897],
 [5808013399864856453914719734275021],
 [1607747922471605773731368605265018]])
# -

res64 = np.linalg.solve(qra.astype('float64'), qrb.astype('float64'))

# +
rr = res64.astype('int')

r3 = qrb[:3,:]-qra[:3, :].dot(rr)
dres = np.linalg.solve(qra[:3,:3].astype('float'), r3.astype('float')).astype('int')
res2 = rr + np.asarray(dres.tolist()+[[0],[0],[0]])
qra.dot(res2)-qrb
# -

res2[:3].sum()


